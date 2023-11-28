use std::ffi::c_void;
use libc::{c_char, size_t};

pub extern "C" fn bzero_c(ptr: *mut c_void, len: size_t) -> *mut c_void {
    if ptr.is_null() {
        return ptr
    }
    for i in 0..len {
        unsafe {
            *((ptr as *mut c_char).add(i)) = 0;
        }
    }
    ptr
}