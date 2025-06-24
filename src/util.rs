// SPDX-License-Identifier: (Apache-2.0 OR MIT)

macro_rules! py_is {
    ($x:expr, $y:expr) => {
        unsafe { $x == $y }
    };
}

macro_rules! ob_type {
    ($obj:expr) => {
        unsafe { (*$obj.cast::<pyo3::ffi::PyObject>()).ob_type }
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
