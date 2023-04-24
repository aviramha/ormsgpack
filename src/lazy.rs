use std::cell::UnsafeCell;
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};

use pyo3::Python;

pub struct ThreadSafeLazy<T, F = fn() -> T> {
    /// A flag which indicates whether the value has been initialized.
    initialized: AtomicBool,
    /// The inner value. This is wrapped in an `UnsafeCell` to allow for interior mutability.
    cell: UnsafeCell<Option<T>>,
    /// The initializer function. The return value of the function will be stored in the cell.
    initializer: F,
}

impl<T, F> ThreadSafeLazy<T, F> {
    pub const fn new(initializer: F) -> ThreadSafeLazy<T, F> {
        Self {
            initialized: AtomicBool::new(false),
            cell: UnsafeCell::new(None),
            initializer,
        }
    }
}

impl<T, F: Fn() -> T> ThreadSafeLazy<T, F> {
    /// Initializes the value of this lazy cell.
    ///
    /// This function is safe to call concurrently from multiple threads. The function uses the
    /// Python Global Interpreter Lock to ensure that the cell is not written to concurrently.
    fn initialize(&self) {
        Python::with_gil(|_py| {
            // It is possible that the initializer function drops the GIL
            // So by the time we re-require the GIL here, cell might have already been initialized
            // by another thread
            let res = (self.initializer)();

            // If that is the case, then we just drop the value we just computed
            let cell = unsafe { &mut *self.cell.get() };
            if cell.is_some() {
                return;
            }

            *cell = Some(res);
            self.initialized.store(true, Ordering::SeqCst);
        });
    }

    /// Forces the evaluation of this lazy cell and returns a reference to the result.
    pub fn force(self: &ThreadSafeLazy<T, F>) -> &T {
        // If the value has not been initialized yet, we need to do so.
        if !self.initialized.load(Ordering::Relaxed) {
            self.initialize();
        }

        // The value has been initialized, so it is safe to read.
        unsafe { &*self.cell.get() }.as_ref().unwrap()
    }
}

impl<T, F: Fn() -> T> Deref for ThreadSafeLazy<T, F> {
    type Target = T;
    fn deref(&self) -> &T {
        ThreadSafeLazy::force(self)
    }
}
