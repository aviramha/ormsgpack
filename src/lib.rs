// SPDX-License-Identifier: (Apache-2.0 OR MIT)

#![feature(core_intrinsics)]
#![allow(unused_unsafe)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_field_names)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::zero_prefixed_literal)]

#[macro_use]
mod util;

mod deserialize;
mod exc;
mod ffi;
mod opt;
mod serialize;
mod typeref;
mod unicode;

use pyo3::ffi::*;
use std::borrow::Cow;
use std::os::raw::c_char;
use std::ptr::NonNull;

const PACKB_DOC: &str =
    "packb(obj, /, default=None, option=None)\n--\n\nSerialize Python objects to msgpack.\0";
const UNPACKB_DOC: &str =
    "unpackb(obj, /, option=None)\n--\n\nDeserialize msgpack to Python objects.\0";

macro_rules! opt {
    ($mptr:expr, $name:expr, $opt:expr) => {
        unsafe {
            #[cfg(not(target_os = "windows"))]
            PyModule_AddIntConstant($mptr, $name.as_ptr() as *const c_char, $opt as i64);
            #[cfg(target_os = "windows")]
            PyModule_AddIntConstant($mptr, $name.as_ptr() as *const c_char, $opt as i32);
        }
    };
}

#[allow(non_snake_case)]
#[no_mangle]
#[cold]
pub unsafe extern "C" fn PyInit_ormsgpack() -> *mut PyObject {
    let init = PyModuleDef {
        m_base: PyModuleDef_HEAD_INIT,
        m_name: "ormsgpack\0".as_ptr() as *const c_char,
        m_doc: std::ptr::null(),
        m_size: 0,
        m_methods: std::ptr::null_mut(),
        m_slots: std::ptr::null_mut(),
        m_traverse: None,
        m_clear: None,
        m_free: None,
    };

    let mptr = PyModule_Create(Box::into_raw(Box::new(init)));

    let version = env!("CARGO_PKG_VERSION");
    unsafe {
        PyModule_AddObject(
            mptr,
            "__version__\0".as_ptr() as *const c_char,
            PyUnicode_FromStringAndSize(version.as_ptr() as *const c_char, version.len() as isize),
        )
    };

    let wrapped_packb: PyMethodDef;
    let wrapped_unpackb: PyMethodDef;

    #[cfg(python37)]
    {
        wrapped_packb = PyMethodDef {
            ml_name: "packb\0".as_ptr() as *const c_char,
            ml_meth: Some(unsafe {
                std::mem::transmute::<pyo3::ffi::_PyCFunctionFastWithKeywords, PyCFunction>(packb)
            }),
            ml_flags: pyo3::ffi::METH_FASTCALL | METH_KEYWORDS,
            ml_doc: PACKB_DOC.as_ptr() as *const c_char,
        };
    }

    #[cfg(not(python37))]
    {
        wrapped_packb = PyMethodDef {
            ml_name: "packb\0".as_ptr() as *const c_char,
            ml_meth: Some(unsafe {
                std::mem::transmute::<PyCFunctionWithKeywords, PyCFunction>(packb)
            }),
            ml_flags: METH_VARARGS | METH_KEYWORDS,
            ml_doc: PACKB_DOC.as_ptr() as *const c_char,
        };
    }

    unsafe {
        PyModule_AddObject(
            mptr,
            "packb\0".as_ptr() as *const c_char,
            PyCFunction_NewEx(
                Box::into_raw(Box::new(wrapped_packb)),
                std::ptr::null_mut(),
                PyUnicode_InternFromString("ormsgpack\0".as_ptr() as *const c_char),
            ),
        )
    };

    #[cfg(python37)]
    {
        wrapped_unpackb = PyMethodDef {
            ml_name: "unpackb\0".as_ptr() as *const c_char,
            ml_meth: Some(unsafe {
                std::mem::transmute::<pyo3::ffi::_PyCFunctionFastWithKeywords, PyCFunction>(unpackb)
            }),
            ml_flags: pyo3::ffi::METH_FASTCALL | METH_KEYWORDS,
            ml_doc: UNPACKB_DOC.as_ptr() as *const c_char,
        };
    }

    #[cfg(not(python37))]
    {
        wrapped_unpackb = PyMethodDef {
            ml_name: "unpackb\0".as_ptr() as *const c_char,
            ml_meth: Some(unsafe {
                std::mem::transmute::<PyCFunctionWithKeywords, PyCFunction>(unpackb)
            }),
            ml_flags: METH_VARARGS | METH_KEYWORDS,
            ml_doc: UNPACKB_DOC.as_ptr() as *const c_char,
        };
    }

    unsafe {
        PyModule_AddObject(
            mptr,
            "unpackb\0".as_ptr() as *const c_char,
            PyCFunction_NewEx(
                Box::into_raw(Box::new(wrapped_unpackb)),
                std::ptr::null_mut(),
                PyUnicode_InternFromString("ormsgpack\0".as_ptr() as *const c_char),
            ),
        )
    };

    opt!(mptr, "OPT_NAIVE_UTC\0", opt::NAIVE_UTC);
    opt!(mptr, "OPT_NON_STR_KEYS\0", opt::NON_STR_KEYS);
    opt!(mptr, "OPT_OMIT_MICROSECONDS\0", opt::OMIT_MICROSECONDS);
    opt!(
        mptr,
        "OPT_PASSTHROUGH_DATACLASS\0",
        opt::PASSTHROUGH_DATACLASS
    );
    opt!(
        mptr,
        "OPT_PASSTHROUGH_DATETIME\0",
        opt::PASSTHROUGH_DATETIME
    );
    opt!(
        mptr,
        "OPT_PASSTHROUGH_SUBCLASS\0",
        opt::PASSTHROUGH_SUBCLASS
    );
    opt!(mptr, "OPT_SERIALIZE_NUMPY\0", opt::SERIALIZE_NUMPY);
    opt!(mptr, "OPT_SERIALIZE_PYDANTIC\0", opt::SERIALIZE_PYDANTIC);
    opt!(mptr, "OPT_UTC_Z\0", opt::UTC_Z);

    typeref::init_typerefs();

    unsafe {
        PyModule_AddObject(
            mptr,
            "MsgpackDecodeError\0".as_ptr() as *const c_char,
            typeref::MsgpackDecodeError,
        );
        PyModule_AddObject(
            mptr,
            "MsgpackEncodeError\0".as_ptr() as *const c_char,
            typeref::MsgpackEncodeError,
        )
    };

    mptr
}

#[cold]
#[inline(never)]
fn raise_unpackb_exception(err: deserialize::DeserializeError) -> *mut PyObject {
    let msg = err.message;
    unsafe {
        let err_msg =
            PyUnicode_FromStringAndSize(msg.as_ptr() as *const c_char, msg.len() as isize);
        let args = PyTuple_New(1);
        PyTuple_SET_ITEM(args, 0, err_msg);
        PyErr_SetObject(typeref::MsgpackDecodeError, args);
        Py_DECREF(args);
    };
    std::ptr::null_mut()
}

#[cold]
#[inline(never)]
fn raise_packb_exception(msg: Cow<str>) -> *mut PyObject {
    unsafe {
        let err_msg =
            PyUnicode_FromStringAndSize(msg.as_ptr() as *const c_char, msg.len() as isize);
        PyErr_SetObject(typeref::MsgpackEncodeError, err_msg);
        Py_DECREF(err_msg);
    };
    std::ptr::null_mut()
}

#[cfg(python37)]
#[no_mangle]
pub unsafe extern "C" fn unpackb(
    _self: *mut PyObject,
    args: *const *mut PyObject,
    nargs: Py_ssize_t,
    kwnames: *mut PyObject,
) -> *mut PyObject {
    let mut optsptr: Option<NonNull<PyObject>> = None;

    let num_args = pyo3::ffi::PyVectorcall_NARGS(nargs as usize);
    if unlikely!(num_args != 1) {
        let msg;
        if num_args > 1 {
            msg = Cow::Borrowed("unpackb() accepts only 1 positional argument");
        } else {
            msg = Cow::Borrowed("unpackb() missing 1 required positional argument: 'obj'")
        }
        return raise_unpackb_exception(deserialize::DeserializeError::new(msg));
    }
    if !kwnames.is_null() {
        let tuple_size = PyTuple_GET_SIZE(kwnames);
        if tuple_size > 0 {
            for i in 0..=tuple_size - 1 {
                let arg = PyTuple_GET_ITEM(kwnames, i as Py_ssize_t);
                if arg == typeref::OPTION {
                    optsptr = Some(NonNull::new_unchecked(*args.offset(num_args + i)));
                } else {
                    return raise_unpackb_exception(deserialize::DeserializeError::new(
                        Cow::Borrowed("unpackb() got an unexpected keyword argument"),
                    ));
                }
            }
        }
    }

    let mut optsbits: i32 = 0;
    if let Some(opts) = optsptr {
        if (*opts.as_ptr()).ob_type != typeref::INT_TYPE {
            return raise_unpackb_exception(deserialize::DeserializeError::new(Cow::Borrowed(
                "Invalid opts",
            )));
        }
        optsbits = PyLong_AsLong(optsptr.unwrap().as_ptr()) as i32;
        if !(0..=opt::MAX_UNPACKB_OPT).contains(&optsbits) {
            return raise_unpackb_exception(deserialize::DeserializeError::new(Cow::Borrowed(
                "Invalid opts",
            )));
        }
    }

    match crate::deserialize::deserialize(*args, optsbits as opt::Opt) {
        Ok(val) => val.as_ptr(),
        Err(err) => raise_unpackb_exception(err),
    }
}

#[cfg(not(python37))]
#[no_mangle]
pub unsafe extern "C" fn unpackb(
    _self: *mut PyObject,
    args: *mut PyObject,
    kwds: *mut PyObject,
) -> *mut PyObject {
    let mut optsptr: Option<NonNull<PyObject>> = None;

    let obj = PyTuple_GET_ITEM(args, 0);

    let num_args = PyTuple_GET_SIZE(args);
    if unlikely!(num_args != 1) {
        let msg;
        if num_args > 1 {
            msg = Cow::Borrowed("unpackb() accepts only 1 positional argument");
        } else {
            msg = Cow::Borrowed("unpackb() missing 1 required positional argument: 'obj'")
        }
        return raise_unpackb_exception(deserialize::DeserializeError::new(msg));
    }

    if !kwds.is_null() {
        let len = unsafe { crate::ffi::PyDict_GET_SIZE(kwds) as usize };
        let mut pos = 0isize;
        let mut arg: *mut PyObject = std::ptr::null_mut();
        let mut val: *mut PyObject = std::ptr::null_mut();
        for _ in 0..=len - 1 {
            unsafe { _PyDict_Next(kwds, &mut pos, &mut arg, &mut val, std::ptr::null_mut()) };
            if arg == typeref::OPTION {
                optsptr = Some(NonNull::new_unchecked(val));
            } else if arg.is_null() {
                break;
            } else {
                return raise_unpackb_exception(deserialize::DeserializeError::new(Cow::Borrowed(
                    "unpackb() got an unexpected keyword argument",
                )));
            }
        }
    }

    let mut optsbits: i32 = 0;
    if let Some(opts) = optsptr {
        if (*opts.as_ptr()).ob_type != typeref::INT_TYPE {
            return raise_unpackb_exception(deserialize::DeserializeError::new(Cow::Borrowed(
                "Invalid opts",
            )));
        }
        optsbits = PyLong_AsLong(optsptr.unwrap().as_ptr()) as i32;
        if optsbits < 0 || optsbits > opt::MAX_UNPACKB_OPT {
            return raise_unpackb_exception(deserialize::DeserializeError::new(Cow::Borrowed(
                "Invalid opts",
            )));
        }
    }

    match crate::deserialize::deserialize(obj, optsbits as opt::Opt) {
        Ok(val) => val.as_ptr(),
        Err(err) => raise_unpackb_exception(err),
    }
}

#[cfg(python37)]
#[no_mangle]
pub unsafe extern "C" fn packb(
    _self: *mut PyObject,
    args: *const *mut PyObject,
    nargs: Py_ssize_t,
    kwnames: *mut PyObject,
) -> *mut PyObject {
    let mut default: Option<NonNull<PyObject>> = None;
    let mut optsptr: Option<NonNull<PyObject>> = None;

    let num_args = pyo3::ffi::PyVectorcall_NARGS(nargs as usize);
    if unlikely!(num_args == 0) {
        return raise_packb_exception(Cow::Borrowed(
            "packb() missing 1 required positional argument: 'obj'",
        ));
    }
    if num_args & 2 == 2 {
        default = Some(NonNull::new_unchecked(*args.offset(1)));
    }
    if num_args & 3 == 3 {
        optsptr = Some(NonNull::new_unchecked(*args.offset(2)));
    }
    if !kwnames.is_null() {
        let tuple_size = PyTuple_GET_SIZE(kwnames);
        if tuple_size > 0 {
            for i in 0..=tuple_size - 1 {
                let arg = PyTuple_GET_ITEM(kwnames, i as Py_ssize_t);
                if arg == typeref::DEFAULT {
                    if unlikely!(num_args & 2 == 2) {
                        return raise_packb_exception(Cow::Borrowed(
                            "packb() got multiple values for argument: 'default'",
                        ));
                    }
                    default = Some(NonNull::new_unchecked(*args.offset(num_args + i)));
                } else if arg == typeref::OPTION {
                    if unlikely!(num_args & 3 == 3) {
                        return raise_packb_exception(Cow::Borrowed(
                            "packb() got multiple values for argument: 'option'",
                        ));
                    }
                    optsptr = Some(NonNull::new_unchecked(*args.offset(num_args + i)));
                } else {
                    return raise_packb_exception(Cow::Borrowed(
                        "packb() got an unexpected keyword argument",
                    ));
                }
            }
        }
    }

    let mut optsbits: i32 = 0;
    if let Some(opts) = optsptr {
        if (*opts.as_ptr()).ob_type != typeref::INT_TYPE {
            return raise_packb_exception(Cow::Borrowed("Invalid opts"));
        }
        optsbits = PyLong_AsLong(optsptr.unwrap().as_ptr()) as i32;
        if !(0..=opt::MAX_PACKB_OPT).contains(&optsbits) {
            return raise_packb_exception(Cow::Borrowed("Invalid opts"));
        }
    }

    match crate::serialize::serialize(*args, default, optsbits as opt::Opt) {
        Ok(val) => val.as_ptr(),
        Err(err) => raise_packb_exception(Cow::Borrowed(&err)),
    }
}

#[cfg(not(python37))]
#[no_mangle]
pub unsafe extern "C" fn packb(
    _self: *mut PyObject,
    args: *mut PyObject,
    kwds: *mut PyObject,
) -> *mut PyObject {
    let mut default: Option<NonNull<PyObject>> = None;
    let mut optsptr: Option<NonNull<PyObject>> = None;

    let obj = PyTuple_GET_ITEM(args, 0);

    let num_args = PyTuple_GET_SIZE(args);
    if unlikely!(num_args == 0) {
        return raise_packb_exception(Cow::Borrowed(
            "packb() missing 1 required positional argument: 'obj'",
        ));
    }
    if num_args & 2 == 2 {
        default = Some(NonNull::new_unchecked(PyTuple_GET_ITEM(args, 1)));
    }
    if num_args & 3 == 3 {
        optsptr = Some(NonNull::new_unchecked(PyTuple_GET_ITEM(args, 2)));
    }

    if !kwds.is_null() {
        let len = unsafe { crate::ffi::PyDict_GET_SIZE(kwds) as usize };
        let mut pos = 0isize;
        let mut arg: *mut PyObject = std::ptr::null_mut();
        let mut val: *mut PyObject = std::ptr::null_mut();
        for _ in 0..=len - 1 {
            unsafe { _PyDict_Next(kwds, &mut pos, &mut arg, &mut val, std::ptr::null_mut()) };
            if arg == typeref::DEFAULT {
                if unlikely!(num_args & 2 == 2) {
                    return raise_packb_exception(Cow::Borrowed(
                        "packb() got multiple values for argument: 'default'",
                    ));
                }
                default = Some(NonNull::new_unchecked(val));
            } else if arg == typeref::OPTION {
                if unlikely!(num_args & 3 == 3) {
                    return raise_packb_exception(Cow::Borrowed(
                        "packb() got multiple values for argument: 'option'",
                    ));
                }
                optsptr = Some(NonNull::new_unchecked(val));
            } else if arg.is_null() {
                break;
            } else {
                return raise_packb_exception(Cow::Borrowed(
                    "packb() got an unexpected keyword argument",
                ));
            }
        }
    }

    let mut optsbits: i32 = 0;
    if let Some(opts) = optsptr {
        if (*opts.as_ptr()).ob_type != typeref::INT_TYPE {
            return raise_packb_exception(Cow::Borrowed("Invalid opts"));
        }
        optsbits = PyLong_AsLong(optsptr.unwrap().as_ptr()) as i32;
        if optsbits < 0 || optsbits > opt::MAX_PACKB_OPT {
            return raise_packb_exception(Cow::Borrowed("Invalid opts"));
        }
    }

    match crate::serialize::serialize(obj, default, optsbits as opt::Opt) {
        Ok(val) => val.as_ptr(),
        Err(err) => raise_packb_exception(Cow::Owned(err)),
    }
}
