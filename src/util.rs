// SPDX-License-Identifier: (Apache-2.0 OR MIT)

macro_rules! py_is {
    ($x:expr, $y:expr) => {
        unsafe { $x == $y }
    };
}

macro_rules! ob_type {
    ($obj:expr) => {
        unsafe { (*($obj as *mut pyo3::ffi::PyObject)).ob_type }
    };
}

macro_rules! err {
    ($msg:expr) => {
        return Err(serde::ser::Error::custom($msg))
    };
}

#[cfg(feature = "unstable-simd")]
macro_rules! unlikely {
    ($exp:expr) => {
        core::intrinsics::unlikely($exp)
    };
}

#[cfg(not(feature = "unstable-simd"))]
macro_rules! unlikely {
    ($exp:expr) => {
        $exp
    };
}

macro_rules! nonnull {
    ($exp:expr) => {
        unsafe { std::ptr::NonNull::new_unchecked($exp) }
    };
}

macro_rules! str_from_slice {
    ($ptr:expr, $size:expr) => {
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts($ptr, $size as usize)) }
    };
}

macro_rules! ffi {
    ($fn:ident()) => {
        unsafe { pyo3::ffi::$fn() }
    };

    ($fn:ident($obj1:expr)) => {
        unsafe { pyo3::ffi::$fn($obj1) }
    };

    ($fn:ident($obj1:expr, $obj2:expr)) => {
        unsafe { pyo3::ffi::$fn($obj1, $obj2) }
    };

    ($fn:ident($obj1:expr, $obj2:expr, $obj3:expr)) => {
        unsafe { pyo3::ffi::$fn($obj1, $obj2, $obj3) }
    };

    ($fn:ident($obj1:expr, $obj2:expr, $obj3:expr, $obj4:expr)) => {
        unsafe { pyo3::ffi::$fn($obj1, $obj2, $obj3, $obj4) }
    };
}

#[cfg(not(Py_3_12))]
macro_rules! pydict_contains {
    ($obj1:expr, $obj2:expr) => {
        unsafe { pyo3::ffi::PyDict_Contains((*$obj1).tp_dict, $obj2) == 1 }
    };
}

#[cfg(Py_3_12)]
macro_rules! pydict_contains {
    ($obj1:expr, $obj2:expr) => {
        unsafe { pyo3::ffi::PyDict_Contains(pyo3::ffi::PyType_GetDict($obj1), $obj2) == 1 }
    };
}
