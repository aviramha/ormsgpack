// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use pyo3::ffi::*;
use std::os::raw::c_int;

mod int;
mod unicode;

pub use unicode::*;

#[inline(always)]
pub unsafe fn pybytes_as_mut_u8(op: *mut PyObject) -> *mut u8 {
    (*op.cast::<PyBytesObject>()).ob_sval.as_mut_ptr() as *mut u8
}

#[inline(always)]
pub unsafe fn pydict_new_presized(len: Py_ssize_t) -> *mut PyObject {
    _PyDict_NewPresized(len)
}

#[inline(always)]
pub unsafe fn pydict_size(mp: *mut PyObject) -> Py_ssize_t {
    Py_SIZE(mp)
}

#[inline(always)]
pub unsafe fn pydict_set_item_known_hash(
    mp: *mut PyObject,
    key: *mut PyObject,
    item: *mut PyObject,
    hash: Py_hash_t,
) -> c_int {
    _PyDict_SetItem_KnownHash(mp, key, item, hash)
}

#[inline(always)]
pub unsafe fn pyobject_call_method_no_args(
    self_: *mut PyObject,
    name: *mut PyObject,
) -> *mut PyObject {
    PyObject_CallMethodNoArgs(self_, name)
}

#[inline(always)]
pub unsafe fn pyobject_call_method_one_arg(
    self_: *mut PyObject,
    name: *mut PyObject,
    arg: *mut PyObject,
) -> *mut PyObject {
    PyObject_CallMethodOneArg(self_, name, arg)
}

#[inline(always)]
pub unsafe fn pytuple_get_item(op: *mut PyObject, i: Py_ssize_t) -> *mut PyObject {
    PyTuple_GET_ITEM(op, i)
}

#[inline(always)]
pub unsafe fn pytuple_set_item(op: *mut PyObject, i: Py_ssize_t, v: *mut PyObject) {
    PyTuple_SET_ITEM(op, i, v)
}
