use std::ffi::c_void;
use libc::{c_char, c_int, c_uchar, size_t};

pub extern "C" fn memccpy_c(dst: *mut c_void, src: *mut c_void, c: c_int, len: size_t) -> *mut c_void {
    if dst.is_null() || src.is_null() {
        return std::ptr::null_mut()
    }
    for i in 0..len {
        unsafe {
            if *(dst.add(i) as *mut c_uchar)  == (c as c_uchar) {
                return dst.add(i + 1);
            }
            *((dst as *mut c_char).add(i)) = *((src as *mut c_char).add(i));
        }
    }
    return std::ptr::null_mut()
}