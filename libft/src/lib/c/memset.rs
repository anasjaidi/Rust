use std::ffi::c_void;
use libc::{c_char, c_int, size_t};

pub extern "C" fn mem_set_c(ptr: *mut c_void, c: c_int, len: size_t) -> *mut c_void {
    unsafe {
        if ptr.is_null() {
            return ptr
        }
        for i in 0..len {
            *((ptr as *mut c_char).add(i))  = c as c_char;
        }
    }
    ptr
}