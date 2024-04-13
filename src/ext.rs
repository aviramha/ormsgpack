use pyo3::ffi::*;
use std::os::raw::{c_char, c_int, c_uint, c_void};
use std::ptr::null_mut;

#[repr(C)]
pub struct PyExt {
    pub ob_base: PyObject,
    pub tag: *mut PyObject,
    pub data: *mut PyObject,
}

#[no_mangle]
unsafe extern "C" fn ext_new(
    subtype: *mut PyTypeObject,
    args: *mut PyObject,
    kwds: *mut PyObject,
) -> *mut PyObject {
    if Py_SIZE(args) != 2 || !kwds.is_null() {
        PyErr_SetString(
            PyExc_TypeError,
            "Ext.__new__() takes 2 positional arguments\0".as_ptr() as *const c_char,
        );
        return null_mut();
    }
    let tag = PyTuple_GET_ITEM(args, 0);
    if PyLong_Check(tag) == 0 {
        PyErr_SetString(
            PyExc_TypeError,
            "Ext.__new__() first argument must be int\0".as_ptr() as *const c_char,
        );
        return null_mut();
    }
    let data = PyTuple_GET_ITEM(args, 1);
    if PyBytes_Check(data) == 0 {
        PyErr_SetString(
            PyExc_TypeError,
            "Ext.__new__() second argument must be bytes\0".as_ptr() as *const c_char,
        );
        return null_mut();
    }
    let obj = (*subtype).tp_alloc.unwrap()(subtype, 0);
    Py_INCREF(tag);
    (*(obj as *mut PyExt)).tag = tag;
    Py_INCREF(data);
    (*(obj as *mut PyExt)).data = data;
    obj
}

#[no_mangle]
unsafe extern "C" fn ext_dealloc(op: *mut PyObject) {
    Py_DECREF((*(op as *mut PyExt)).tag);
    Py_DECREF((*(op as *mut PyExt)).data);
    (*ob_type!(op)).tp_free.unwrap()(op as *mut c_void);
}

pub unsafe fn create_ext_type() -> *mut PyTypeObject {
    let mut slots: [PyType_Slot; 3] = [
        PyType_Slot {
            slot: Py_tp_new,
            pfunc: ext_new as *mut c_void,
        },
        PyType_Slot {
            slot: Py_tp_dealloc,
            pfunc: ext_dealloc as *mut c_void,
        },
        PyType_Slot {
            slot: 0,
            pfunc: null_mut(),
        },
    ];
    let mut spec = PyType_Spec {
        name: "ormsgpack.Ext\0".as_ptr() as *const c_char,
        basicsize: std::mem::size_of::<PyExt>() as c_int,
        itemsize: 0,
        flags: Py_TPFLAGS_DEFAULT as c_uint,
        slots: slots.as_mut_ptr(),
    };
    PyType_FromSpec(&mut spec) as *mut PyTypeObject
}
