// SPDX-License-Identifier: (Apache-2.0 OR MIT)
#![cfg_attr(feature = "unstable-simd", feature(core_intrinsics))]
#![allow(internal_features)]
#![allow(unused_unsafe)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::redundant_field_names)]
#![allow(clippy::upper_case_acronyms)]
#![allow(clippy::zero_prefixed_literal)]

#[macro_use]
mod util;

mod deserialize;
mod exc;
mod ext;
mod ffi;
mod opt;
mod serialize;
mod typeref;
mod unicode;

use pyo3::ffi::*;
use std::borrow::Cow;
use std::os::raw::c_char;
use std::os::raw::c_int;
use std::os::raw::c_long;
use std::os::raw::c_void;
use std::ptr::NonNull;

const PACKB_DOC: &str =
    "packb(obj, /, default=None, option=None)\n--\n\nSerialize Python objects to msgpack.\0";
const UNPACKB_DOC: &str =
    "unpackb(obj, /, ext_hook=None, option=None)\n--\n\nDeserialize msgpack to Python objects.\0";

macro_rules! module_add_object {
    ($mptr: expr, $name: expr, $object:expr) => {
        PyModule_AddObject($mptr, $name.as_ptr() as *const c_char, $object);
    };
}

macro_rules! module_add_int {
    ($mptr:expr, $name:expr, $int:expr) => {
        PyModule_AddIntConstant($mptr, $name.as_ptr() as *const c_char, $int as c_long);
    };
}

#[allow(non_snake_case)]
#[no_mangle]
#[cold]
pub unsafe extern "C" fn PyInit_ormsgpack() -> *mut PyModuleDef {
    let methods: Box<[PyMethodDef; 3]> = Box::new([
        PyMethodDef {
            ml_name: "packb\0".as_ptr() as *const c_char,
            ml_meth: PyMethodDefPointer {
                _PyCFunctionFastWithKeywords: packb,
            },
            ml_flags: METH_FASTCALL | METH_KEYWORDS,
            ml_doc: PACKB_DOC.as_ptr() as *const c_char,
        },
        PyMethodDef {
            ml_name: "unpackb\0".as_ptr() as *const c_char,
            ml_meth: PyMethodDefPointer {
                _PyCFunctionFastWithKeywords: unpackb,
            },
            ml_flags: METH_FASTCALL | METH_KEYWORDS,
            ml_doc: UNPACKB_DOC.as_ptr() as *const c_char,
        },
        PyMethodDef::zeroed(),
    ]);

    let slots: Box<[PyModuleDef_Slot; 2]> = Box::new([
        PyModuleDef_Slot {
            slot: Py_mod_exec,
            value: ormsgpack_exec as *mut c_void,
        },
        PyModuleDef_Slot {
            slot: 0,
            value: std::ptr::null_mut(),
        },
    ]);

    let init = Box::new(PyModuleDef {
        m_base: PyModuleDef_HEAD_INIT,
        m_name: "ormsgpack\0".as_ptr() as *const c_char,
        m_doc: std::ptr::null(),
        m_size: 0,
        m_methods: Box::into_raw(methods) as *mut PyMethodDef,
        m_slots: Box::into_raw(slots) as *mut PyModuleDef_Slot,
        m_traverse: None,
        m_clear: None,
        m_free: None,
    });
    let init_ptr = Box::into_raw(init);
    PyModuleDef_Init(init_ptr);
    init_ptr
}

#[allow(non_snake_case)]
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ormsgpack_exec(mptr: *mut PyObject) -> c_int {
    let version = env!("CARGO_PKG_VERSION");
    module_add_object!(
        mptr,
        "__version__\0",
        PyUnicode_FromStringAndSize(version.as_ptr() as *const c_char, version.len() as isize)
    );

    module_add_int!(mptr, "OPT_NAIVE_UTC\0", opt::NAIVE_UTC);
    module_add_int!(mptr, "OPT_NON_STR_KEYS\0", opt::NON_STR_KEYS);
    module_add_int!(mptr, "OPT_OMIT_MICROSECONDS\0", opt::OMIT_MICROSECONDS);
    module_add_int!(mptr, "OPT_PASSTHROUGH_BIG_INT\0", opt::PASSTHROUGH_BIG_INT);
    module_add_int!(
        mptr,
        "OPT_PASSTHROUGH_DATACLASS\0",
        opt::PASSTHROUGH_DATACLASS
    );
    module_add_int!(
        mptr,
        "OPT_PASSTHROUGH_DATETIME\0",
        opt::PASSTHROUGH_DATETIME
    );
    module_add_int!(
        mptr,
        "OPT_PASSTHROUGH_SUBCLASS\0",
        opt::PASSTHROUGH_SUBCLASS
    );
    module_add_int!(mptr, "OPT_SERIALIZE_NUMPY\0", opt::SERIALIZE_NUMPY);
    module_add_int!(mptr, "OPT_SERIALIZE_PYDANTIC\0", opt::SERIALIZE_PYDANTIC);
    module_add_int!(mptr, "OPT_PASSTHROUGH_TUPLE\0", opt::PASSTHROUGH_TUPLE);
    module_add_int!(mptr, "OPT_SORT_KEYS\0", opt::SORT_KEYS);
    module_add_int!(mptr, "OPT_UTC_Z\0", opt::UTC_Z);

    typeref::init_typerefs();

    module_add_object!(mptr, "MsgpackDecodeError\0", typeref::MsgpackDecodeError);
    module_add_object!(mptr, "MsgpackEncodeError\0", typeref::MsgpackEncodeError);
    module_add_object!(mptr, "Ext\0", typeref::EXT_TYPE as *mut PyObject);

    0
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

#[no_mangle]
pub unsafe extern "C" fn unpackb(
    _self: *mut PyObject,
    args: *const *mut PyObject,
    nargs: Py_ssize_t,
    kwnames: *mut PyObject,
) -> *mut PyObject {
    let mut ext_hook: Option<NonNull<PyObject>> = None;
    let mut optsptr: Option<NonNull<PyObject>> = None;

    let num_args = PyVectorcall_NARGS(nargs as usize);
    if unlikely!(num_args != 1) {
        let msg = if num_args > 1 {
            Cow::Borrowed("unpackb() accepts only 1 positional argument")
        } else {
            Cow::Borrowed("unpackb() missing 1 required positional argument: 'obj'")
        };
        return raise_unpackb_exception(deserialize::DeserializeError::new(msg));
    }
    if !kwnames.is_null() {
        let tuple_size = PyTuple_GET_SIZE(kwnames);
        if tuple_size > 0 {
            for i in 0..=tuple_size - 1 {
                let arg = PyTuple_GET_ITEM(kwnames, i as Py_ssize_t);
                if arg == typeref::EXT_HOOK {
                    ext_hook = Some(NonNull::new_unchecked(*args.offset(num_args + i)));
                } else if arg == typeref::OPTION {
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
        let ob_type = (*opts.as_ptr()).ob_type;
        if ob_type == typeref::INT_TYPE {
            optsbits = PyLong_AsLong(optsptr.unwrap().as_ptr()) as i32;
            if !(0..=opt::MAX_UNPACKB_OPT).contains(&optsbits) {
                return raise_unpackb_exception(deserialize::DeserializeError::new(Cow::Borrowed(
                    "Invalid opts",
                )));
            }
        } else if ob_type != typeref::NONE_TYPE {
            return raise_unpackb_exception(deserialize::DeserializeError::new(Cow::Borrowed(
                "Invalid opts",
            )));
        }
    }

    match crate::deserialize::deserialize(*args, ext_hook, optsbits as opt::Opt) {
        Ok(val) => val.as_ptr(),
        Err(err) => raise_unpackb_exception(err),
    }
}

#[no_mangle]
pub unsafe extern "C" fn packb(
    _self: *mut PyObject,
    args: *const *mut PyObject,
    nargs: Py_ssize_t,
    kwnames: *mut PyObject,
) -> *mut PyObject {
    let mut default: Option<NonNull<PyObject>> = None;
    let mut optsptr: Option<NonNull<PyObject>> = None;

    let num_args = PyVectorcall_NARGS(nargs as usize);
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
        let ob_type = (*opts.as_ptr()).ob_type;
        if ob_type == typeref::INT_TYPE {
            optsbits = PyLong_AsLong(optsptr.unwrap().as_ptr()) as i32;
            if !(0..=opt::MAX_PACKB_OPT).contains(&optsbits) {
                return raise_packb_exception(Cow::Borrowed("Invalid opts"));
            }
        } else if ob_type != typeref::NONE_TYPE {
            return raise_packb_exception(Cow::Borrowed("Invalid opts"));
        }
    }

    match crate::serialize::serialize(*args, default, optsbits as opt::Opt) {
        Ok(val) => val.as_ptr(),
        Err(err) => raise_packb_exception(Cow::Borrowed(&err)),
    }
}
