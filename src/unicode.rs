// SPDX-License-Identifier: (Apache-2.0 OR MIT)

use crate::typeref::EMPTY_UNICODE;
use core::ffi::c_void;
use pyo3::ffi::*;

// see unicodeobject.h for documentation

const STRIDE_SIZE: usize = 8;

fn is_four_byte(buf: &str) -> bool {
    let as_bytes = buf.as_bytes();
    let len = as_bytes.len();
    unsafe {
        let mut idx = 0;
        while idx < len.saturating_sub(STRIDE_SIZE) {
            let mut val: bool = false;
            val |= *as_bytes.get_unchecked(idx) > 239;
            val |= *as_bytes.get_unchecked(idx + 1) > 239;
            val |= *as_bytes.get_unchecked(idx + 2) > 239;
            val |= *as_bytes.get_unchecked(idx + 3) > 239;
            val |= *as_bytes.get_unchecked(idx + 4) > 239;
            val |= *as_bytes.get_unchecked(idx + 5) > 239;
            val |= *as_bytes.get_unchecked(idx + 6) > 239;
            val |= *as_bytes.get_unchecked(idx + 7) > 239;
            idx += STRIDE_SIZE;
            if val {
                return true;
            }
        }
        let mut ret = false;
        while idx < len {
            ret |= *as_bytes.get_unchecked(idx) > 239;
            idx += 1;
        }
        ret
    }
}

enum PyUnicodeKind {
    Ascii,
    OneByte,
    TwoByte,
    FourByte,
}

fn find_str_kind(buf: &str, num_chars: usize) -> PyUnicodeKind {
    if buf.len() == num_chars {
        PyUnicodeKind::Ascii
    } else if is_four_byte(buf) {
        PyUnicodeKind::FourByte
    } else if encoding_rs::mem::is_str_latin1(buf) {
        PyUnicodeKind::OneByte
    } else {
        PyUnicodeKind::TwoByte
    }
}

pub fn unicode_from_str(buf: &str) -> *mut PyObject {
    if buf.is_empty() {
        ffi!(Py_INCREF(EMPTY_UNICODE));
        unsafe { EMPTY_UNICODE }
    } else {
        let num_chars = bytecount::num_chars(buf.as_bytes());
        match find_str_kind(buf, num_chars) {
            PyUnicodeKind::Ascii => pyunicode_ascii(buf),
            PyUnicodeKind::OneByte => pyunicode_onebyte(buf, num_chars),
            PyUnicodeKind::TwoByte => pyunicode_twobyte(buf, num_chars),
            PyUnicodeKind::FourByte => pyunicode_fourbyte(buf, num_chars),
        }
    }
}

fn pyunicode_ascii(buf: &str) -> *mut PyObject {
    unsafe {
        let ptr = ffi!(PyUnicode_New(buf.len() as isize, 127));
        let data_ptr = ptr.cast::<PyASCIIObject>().offset(1) as *mut u8;
        std::ptr::copy_nonoverlapping(buf.as_ptr(), data_ptr, buf.len());
        std::ptr::write(data_ptr.add(buf.len()), 0);
        ptr
    }
}

#[cold]
#[inline(never)]
fn pyunicode_onebyte(buf: &str, num_chars: usize) -> *mut PyObject {
    unsafe {
        let ptr = ffi!(PyUnicode_New(num_chars as isize, 255));
        let mut data_ptr = ptr.cast::<PyCompactUnicodeObject>().offset(1) as *mut u8;
        for each in buf.chars() {
            std::ptr::write(data_ptr, each as u8);
            data_ptr = data_ptr.offset(1);
        }
        std::ptr::write(data_ptr, 0);
        ptr
    }
}

fn pyunicode_twobyte(buf: &str, num_chars: usize) -> *mut PyObject {
    unsafe {
        let ptr = ffi!(PyUnicode_New(num_chars as isize, 65535));
        let mut data_ptr = ptr.cast::<PyCompactUnicodeObject>().offset(1) as *mut u16;
        for each in buf.chars() {
            std::ptr::write(data_ptr, each as u16);
            data_ptr = data_ptr.offset(1);
        }
        std::ptr::write(data_ptr, 0);
        ptr
    }
}

fn pyunicode_fourbyte(buf: &str, num_chars: usize) -> *mut PyObject {
    unsafe {
        let ptr = ffi!(PyUnicode_New(num_chars as isize, 1114111));
        let mut data_ptr = ptr.cast::<PyCompactUnicodeObject>().offset(1) as *mut u32;
        for each in buf.chars() {
            std::ptr::write(data_ptr, each as u32);
            data_ptr = data_ptr.offset(1);
        }
        std::ptr::write(data_ptr, 0);
        ptr
    }
}

#[inline]
pub fn hash_str(op: *mut PyObject) -> Py_hash_t {
    unsafe {
        let data_ptr: *mut c_void = if (*op.cast::<PyASCIIObject>()).compact() == 1
            && (*op.cast::<PyASCIIObject>()).ascii() == 1
        {
            (op as *mut PyASCIIObject).offset(1) as *mut c_void
        } else {
            (op as *mut PyCompactUnicodeObject).offset(1) as *mut c_void
        };
        let num_bytes =
            (*(op as *mut PyASCIIObject)).length * ((*(op as *mut PyASCIIObject)).kind()) as isize;
        let hash = _Py_HashBytes(data_ptr, num_bytes);
        (*op.cast::<PyASCIIObject>()).hash = hash;
        hash
    }
}

#[inline(never)]
pub fn unicode_to_str_via_ffi(op: *mut PyObject) -> Option<&'static str> {
    let mut str_size: Py_ssize_t = 0;
    let ptr = ffi!(PyUnicode_AsUTF8AndSize(op, &mut str_size)) as *const u8;
    if unlikely!(ptr.is_null()) {
        None
    } else {
        Some(str_from_slice!(ptr, str_size as usize))
    }
}

#[inline]
pub fn unicode_to_str(op: *mut PyObject) -> Option<&'static str> {
    unsafe {
        if unlikely!((*op.cast::<PyASCIIObject>()).compact() == 0) {
            unicode_to_str_via_ffi(op)
        } else if (*op.cast::<PyASCIIObject>()).ascii() == 1 {
            let ptr = op.cast::<PyASCIIObject>().offset(1) as *const u8;
            let len = (*op.cast::<PyASCIIObject>()).length as usize;
            Some(str_from_slice!(ptr, len))
        } else if (*op.cast::<PyCompactUnicodeObject>()).utf8_length != 0 {
            let ptr = (*op.cast::<PyCompactUnicodeObject>()).utf8 as *const u8;
            let len = (*op.cast::<PyCompactUnicodeObject>()).utf8_length as usize;
            Some(str_from_slice!(ptr, len))
        } else {
            unicode_to_str_via_ffi(op)
        }
    }
}
