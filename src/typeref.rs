// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use ahash::RandomState;
use once_cell::race::OnceBox;
use pyo3::ffi::*;
use std::ffi::CStr;
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
        DATETIME_TYPE = (*PyDateTimeAPI()).DateTimeType;
        DATE_TYPE = (*PyDateTimeAPI()).DateType;
        TIME_TYPE = (*PyDateTimeAPI()).TimeType;
        UUID_TYPE = look_up_type(c"uuid", c"UUID");
        ENUM_TYPE = look_up_type(c"enum", c"EnumMeta");
        FIELD_TYPE = look_up_type(c"dataclasses", c"_FIELD");
        EXT_TYPE = create_ext_type();
        INT_ATTR_STR = PyUnicode_InternFromString(c"int".as_ptr());
        UTCOFFSET_METHOD_STR = PyUnicode_InternFromString(c"utcoffset".as_ptr());
        NORMALIZE_METHOD_STR = PyUnicode_InternFromString(c"normalize".as_ptr());
        CONVERT_METHOD_STR = PyUnicode_InternFromString(c"convert".as_ptr());
        DST_STR = PyUnicode_InternFromString(c"dst".as_ptr());
        DICT_STR = PyUnicode_InternFromString(c"__dict__".as_ptr());
        DATACLASS_FIELDS_STR = PyUnicode_InternFromString(c"__dataclass_fields__".as_ptr());
        SLOTS_STR = PyUnicode_InternFromString(c"__slots__".as_ptr());
        PYDANTIC_FIELDS_STR = PyUnicode_InternFromString(c"__fields__".as_ptr());
        PYDANTIC2_VALIDATOR_STR = PyUnicode_InternFromString(c"__pydantic_validator__".as_ptr());
        FIELD_TYPE_STR = PyUnicode_InternFromString(c"_field_type".as_ptr());
        ARRAY_STRUCT_STR = PyUnicode_InternFromString(c"__array_struct__".as_ptr());
        DTYPE_STR = PyUnicode_InternFromString(c"dtype".as_ptr());
        DESCR_STR = PyUnicode_InternFromString(c"descr".as_ptr());
        VALUE_STR = PyUnicode_InternFromString(c"value".as_ptr());
        DEFAULT = PyUnicode_InternFromString(c"default".as_ptr());
        EXT_HOOK = PyUnicode_InternFromString(c"ext_hook".as_ptr());
        OPTION = PyUnicode_InternFromString(c"option".as_ptr());
        Py_INCREF(PyExc_TypeError);
        MsgpackEncodeError = PyExc_TypeError;
        Py_INCREF(PyExc_ValueError);
        MsgpackDecodeError = PyExc_ValueError;

        HASH_BUILDER.get_or_init(ahash_init);
    });
}

#[cold]
unsafe fn look_up_numpy_type(
    numpy_module_dict: *mut PyObject,
    np_type: &CStr,
) -> *mut PyTypeObject {
    let ptr = PyMapping_GetItemString(numpy_module_dict, np_type.as_ptr());
    ptr as *mut PyTypeObject
}

#[cold]
pub fn load_numpy_types() -> Box<Option<NonNull<NumpyTypes>>> {
    unsafe {
        let numpy = PyImport_ImportModule(c"numpy".as_ptr());
        if numpy.is_null() {
            PyErr_Clear();
            return Box::new(None);
        }

        let numpy_module_dict = PyObject_GenericGetDict(numpy, null_mut());
        let types = Box::new(NumpyTypes {
            array: look_up_numpy_type(numpy_module_dict, c"ndarray"),
            float16: look_up_numpy_type(numpy_module_dict, c"half"),
            float32: look_up_numpy_type(numpy_module_dict, c"float32"),
            float64: look_up_numpy_type(numpy_module_dict, c"float64"),
            int8: look_up_numpy_type(numpy_module_dict, c"int8"),
            int16: look_up_numpy_type(numpy_module_dict, c"int16"),
            int32: look_up_numpy_type(numpy_module_dict, c"int32"),
            int64: look_up_numpy_type(numpy_module_dict, c"int64"),
            uint16: look_up_numpy_type(numpy_module_dict, c"uint16"),
            uint32: look_up_numpy_type(numpy_module_dict, c"uint32"),
            uint64: look_up_numpy_type(numpy_module_dict, c"uint64"),
            uint8: look_up_numpy_type(numpy_module_dict, c"uint8"),
            bool_: look_up_numpy_type(numpy_module_dict, c"bool_"),
            datetime64: look_up_numpy_type(numpy_module_dict, c"datetime64"),
        });
        Py_DECREF(numpy_module_dict);
        Py_DECREF(numpy);
        Box::new(Some(nonnull!(Box::<NumpyTypes>::into_raw(types))))
    }
}

#[cold]
unsafe fn look_up_type(module_name: &CStr, type_name: &CStr) -> *mut PyTypeObject {
    let module = PyImport_ImportModule(module_name.as_ptr());
    let module_dict = PyObject_GenericGetDict(module, null_mut());
    let ptr = PyMapping_GetItemString(module_dict, type_name.as_ptr()) as *mut PyTypeObject;
    Py_DECREF(module_dict);
    Py_DECREF(module);
    ptr
}
