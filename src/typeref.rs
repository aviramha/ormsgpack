// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use ahash::RandomState;
use once_cell::race::OnceBox;
use pyo3::ffi::*;
use std::os::raw::c_char;
use std::ptr::{null_mut, NonNull};
use std::sync::Once;

use crate::ext::create_ext_type;

pub struct NumpyTypes {
    pub array: *mut PyTypeObject,
    pub float64: *mut PyTypeObject,
    pub float32: *mut PyTypeObject,
    pub float16: *mut PyTypeObject,
    pub int64: *mut PyTypeObject,
    pub int32: *mut PyTypeObject,
    pub int16: *mut PyTypeObject,
    pub int8: *mut PyTypeObject,
    pub uint64: *mut PyTypeObject,
    pub uint32: *mut PyTypeObject,
    pub uint16: *mut PyTypeObject,
    pub uint8: *mut PyTypeObject,
    pub bool_: *mut PyTypeObject,
    pub datetime64: *mut PyTypeObject,
}

pub static mut DEFAULT: *mut PyObject = null_mut();
pub static mut EXT_HOOK: *mut PyObject = null_mut();
pub static mut OPTION: *mut PyObject = null_mut();

pub static mut NONE: *mut PyObject = null_mut();
pub static mut TRUE: *mut PyObject = null_mut();
pub static mut FALSE: *mut PyObject = null_mut();
pub static mut EMPTY_UNICODE: *mut PyObject = null_mut();

pub static mut BYTES_TYPE: *mut PyTypeObject = null_mut();
pub static mut BYTEARRAY_TYPE: *mut PyTypeObject = null_mut();
pub static mut MEMORYVIEW_TYPE: *mut PyTypeObject = null_mut();
pub static mut STR_TYPE: *mut PyTypeObject = null_mut();
pub static mut INT_TYPE: *mut PyTypeObject = null_mut();
pub static mut BOOL_TYPE: *mut PyTypeObject = null_mut();
pub static mut FLOAT_TYPE: *mut PyTypeObject = null_mut();
pub static mut LIST_TYPE: *mut PyTypeObject = null_mut();
pub static mut DICT_TYPE: *mut PyTypeObject = null_mut();
pub static mut DATETIME_TYPE: *mut PyTypeObject = null_mut();
pub static mut DATE_TYPE: *mut PyTypeObject = null_mut();
pub static mut TIME_TYPE: *mut PyTypeObject = null_mut();
pub static mut TUPLE_TYPE: *mut PyTypeObject = null_mut();
pub static mut UUID_TYPE: *mut PyTypeObject = null_mut();
pub static mut ENUM_TYPE: *mut PyTypeObject = null_mut();
pub static mut FIELD_TYPE: *mut PyTypeObject = null_mut();
pub static mut EXT_TYPE: *mut PyTypeObject = null_mut();
pub static mut NUMPY_TYPES: OnceBox<Option<NonNull<NumpyTypes>>> = OnceBox::new();

pub static mut UTCOFFSET_METHOD_STR: *mut PyObject = null_mut();
pub static mut NORMALIZE_METHOD_STR: *mut PyObject = null_mut();
pub static mut CONVERT_METHOD_STR: *mut PyObject = null_mut();
pub static mut DST_STR: *mut PyObject = null_mut();

pub static mut DICT_STR: *mut PyObject = null_mut();
pub static mut DATACLASS_FIELDS_STR: *mut PyObject = null_mut();
pub static mut SLOTS_STR: *mut PyObject = null_mut();
pub static mut PYDANTIC_FIELDS_STR: *mut PyObject = null_mut();
pub static mut PYDANTIC2_VALIDATOR_STR: *mut PyObject = null_mut();
pub static mut FIELD_TYPE_STR: *mut PyObject = null_mut();
pub static mut ARRAY_STRUCT_STR: *mut PyObject = null_mut();
pub static mut DTYPE_STR: *mut PyObject = null_mut();
pub static mut DESCR_STR: *mut PyObject = null_mut();
pub static mut VALUE_STR: *mut PyObject = null_mut();
pub static mut INT_ATTR_STR: *mut PyObject = null_mut();

pub static mut HASH_BUILDER: OnceBox<ahash::RandomState> = OnceBox::new();

pub fn ahash_init() -> Box<ahash::RandomState> {
    unsafe {
        debug_assert!(!VALUE_STR.is_null());
        debug_assert!(!DICT_TYPE.is_null());
        debug_assert!(!STR_TYPE.is_null());
        debug_assert!(!BYTES_TYPE.is_null());
        Box::new(RandomState::with_seeds(
            VALUE_STR as u64,
            DICT_TYPE as u64,
            STR_TYPE as u64,
            BYTES_TYPE as u64,
        ))
    }
}

#[allow(non_upper_case_globals)]
pub static mut MsgpackEncodeError: *mut PyObject = null_mut();
#[allow(non_upper_case_globals)]
pub static mut MsgpackDecodeError: *mut PyObject = null_mut();

static INIT: Once = Once::new();

#[cold]
pub fn init_typerefs() {
    INIT.call_once(|| unsafe {
        assert!(crate::deserialize::KEY_MAP
            .set(crate::deserialize::KeyMap::new())
            .is_ok());
        PyDateTime_IMPORT();
        NONE = Py_None();
        TRUE = Py_True();
        FALSE = Py_False();
        EMPTY_UNICODE = PyUnicode_New(0, 255);
        STR_TYPE = &mut PyUnicode_Type;
        BYTES_TYPE = &mut PyBytes_Type;
        BYTEARRAY_TYPE = &mut PyByteArray_Type;
        MEMORYVIEW_TYPE = &mut PyMemoryView_Type;
        DICT_TYPE = &mut PyDict_Type;
        LIST_TYPE = &mut PyList_Type;
        TUPLE_TYPE = &mut PyTuple_Type;
        BOOL_TYPE = &mut PyBool_Type;
        INT_TYPE = &mut PyLong_Type;
        FLOAT_TYPE = &mut PyFloat_Type;
        DATETIME_TYPE = look_up_datetime_type();
        DATE_TYPE = look_up_date_type();
        TIME_TYPE = look_up_time_type();
        UUID_TYPE = look_up_uuid_type();
        ENUM_TYPE = look_up_enum_type();
        FIELD_TYPE = look_up_field_type();
        EXT_TYPE = create_ext_type();
        INT_ATTR_STR = PyUnicode_InternFromString("int\0".as_ptr() as *const c_char);
        UTCOFFSET_METHOD_STR = PyUnicode_InternFromString("utcoffset\0".as_ptr() as *const c_char);
        NORMALIZE_METHOD_STR = PyUnicode_InternFromString("normalize\0".as_ptr() as *const c_char);
        CONVERT_METHOD_STR = PyUnicode_InternFromString("convert\0".as_ptr() as *const c_char);
        DST_STR = PyUnicode_InternFromString("dst\0".as_ptr() as *const c_char);
        DICT_STR = PyUnicode_InternFromString("__dict__\0".as_ptr() as *const c_char);
        DATACLASS_FIELDS_STR =
            PyUnicode_InternFromString("__dataclass_fields__\0".as_ptr() as *const c_char);
        SLOTS_STR = PyUnicode_InternFromString("__slots__\0".as_ptr() as *const c_char);
        PYDANTIC_FIELDS_STR = PyUnicode_InternFromString("__fields__\0".as_ptr() as *const c_char);
        PYDANTIC2_VALIDATOR_STR =
            PyUnicode_InternFromString("__pydantic_validator__\0".as_ptr() as *const c_char);
        FIELD_TYPE_STR = PyUnicode_InternFromString("_field_type\0".as_ptr() as *const c_char);
        ARRAY_STRUCT_STR =
            PyUnicode_InternFromString("__array_struct__\0".as_ptr() as *const c_char);
        DTYPE_STR = PyUnicode_InternFromString("dtype\0".as_ptr() as *const c_char);
        DESCR_STR = PyUnicode_InternFromString("descr\0".as_ptr() as *const c_char);
        VALUE_STR = PyUnicode_InternFromString("value\0".as_ptr() as *const c_char);
        DEFAULT = PyUnicode_InternFromString("default\0".as_ptr() as *const c_char);
        EXT_HOOK = PyUnicode_InternFromString("ext_hook\0".as_ptr() as *const c_char);
        OPTION = PyUnicode_InternFromString("option\0".as_ptr() as *const c_char);
        Py_INCREF(PyExc_TypeError);
        MsgpackEncodeError = PyExc_TypeError;
        Py_INCREF(PyExc_ValueError);
        MsgpackDecodeError = PyExc_ValueError;

        HASH_BUILDER.get_or_init(ahash_init);
    });
}

#[cold]
unsafe fn look_up_numpy_type(numpy_module_dict: *mut PyObject, np_type: &str) -> *mut PyTypeObject {
    let ptr = PyMapping_GetItemString(numpy_module_dict, np_type.as_ptr() as *const c_char);
    Py_XDECREF(ptr);
    ptr as *mut PyTypeObject
}

#[cold]
pub fn load_numpy_types() -> Box<Option<NonNull<NumpyTypes>>> {
    unsafe {
        let numpy = PyImport_ImportModule("numpy\0".as_ptr() as *const c_char);
        if numpy.is_null() {
            PyErr_Clear();
            return Box::new(None);
        }

        let numpy_module_dict = PyObject_GenericGetDict(numpy, null_mut());
        let types = Box::new(NumpyTypes {
            array: look_up_numpy_type(numpy_module_dict, "ndarray\0"),
            float16: look_up_numpy_type(numpy_module_dict, "half\0"),
            float32: look_up_numpy_type(numpy_module_dict, "float32\0"),
            float64: look_up_numpy_type(numpy_module_dict, "float64\0"),
            int8: look_up_numpy_type(numpy_module_dict, "int8\0"),
            int16: look_up_numpy_type(numpy_module_dict, "int16\0"),
            int32: look_up_numpy_type(numpy_module_dict, "int32\0"),
            int64: look_up_numpy_type(numpy_module_dict, "int64\0"),
            uint16: look_up_numpy_type(numpy_module_dict, "uint16\0"),
            uint32: look_up_numpy_type(numpy_module_dict, "uint32\0"),
            uint64: look_up_numpy_type(numpy_module_dict, "uint64\0"),
            uint8: look_up_numpy_type(numpy_module_dict, "uint8\0"),
            bool_: look_up_numpy_type(numpy_module_dict, "bool_\0"),
            datetime64: look_up_numpy_type(numpy_module_dict, "datetime64\0"),
        });
        Py_XDECREF(numpy_module_dict);
        Py_XDECREF(numpy);
        Box::new(Some(nonnull!(Box::<NumpyTypes>::into_raw(types))))
    }
}

#[cold]
unsafe fn look_up_field_type() -> *mut PyTypeObject {
    let module = PyImport_ImportModule("dataclasses\0".as_ptr() as *const c_char);
    let module_dict = PyObject_GenericGetDict(module, null_mut());
    let ptr = PyMapping_GetItemString(module_dict, "_FIELD\0".as_ptr() as *const c_char)
        as *mut PyTypeObject;
    Py_DECREF(module_dict);
    Py_DECREF(module);
    ptr
}

#[cold]
unsafe fn look_up_enum_type() -> *mut PyTypeObject {
    let module = PyImport_ImportModule("enum\0".as_ptr() as *const c_char);
    let module_dict = PyObject_GenericGetDict(module, null_mut());
    let ptr = PyMapping_GetItemString(module_dict, "EnumMeta\0".as_ptr() as *const c_char)
        as *mut PyTypeObject;
    Py_DECREF(module_dict);
    Py_DECREF(module);
    ptr
}

#[cold]
unsafe fn look_up_uuid_type() -> *mut PyTypeObject {
    let uuid_mod = PyImport_ImportModule("uuid\0".as_ptr() as *const c_char);
    let uuid_mod_dict = PyObject_GenericGetDict(uuid_mod, null_mut());
    let uuid = PyMapping_GetItemString(uuid_mod_dict, "NAMESPACE_DNS\0".as_ptr() as *const c_char);
    let ptr = (*uuid).ob_type;
    Py_DECREF(uuid);
    Py_DECREF(uuid_mod_dict);
    Py_DECREF(uuid_mod);
    ptr
}

#[cold]
unsafe fn look_up_datetime_type() -> *mut PyTypeObject {
    let datetime_api = *PyDateTimeAPI();
    let datetime = (datetime_api.DateTime_FromDateAndTime)(
        1970,
        1,
        1,
        0,
        0,
        0,
        0,
        NONE,
        datetime_api.DateTimeType,
    );
    let ptr = (*datetime).ob_type;
    Py_DECREF(datetime);
    ptr
}

#[cold]
unsafe fn look_up_date_type() -> *mut PyTypeObject {
    let datetime_api = *PyDateTimeAPI();
    let date = (datetime_api.Date_FromDate)(1970, 1, 1, datetime_api.DateType);
    let ptr = (*date).ob_type;
    Py_DECREF(date);
    ptr
}

#[cold]
unsafe fn look_up_time_type() -> *mut PyTypeObject {
    let datetime_api = *PyDateTimeAPI();
    let time = (datetime_api.Time_FromTime)(0, 0, 0, 0, NONE, datetime_api.TimeType);
    let ptr = (*time).ob_type;
    Py_DECREF(time);
    ptr
}
