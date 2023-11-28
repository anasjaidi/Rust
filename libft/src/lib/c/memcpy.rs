use std::ffi::c_void;
use libc::{c_char, size_t};

pub extern "C" fn memcpy_c(dst: *mut c_void, src: *mut c_void, len: size_t) -> *mut c_void {
    if dst.is_null() || src.is_null() {
        return dst
    }
    for i in 0..len {
        unsafe {
            *((dst as *mut c_char).add(i)) = *((src as *mut c_char).add(i));
        }
    }
    dst
}