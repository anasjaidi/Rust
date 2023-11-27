// void *memset(void *b, int c, size_t len);

use std::ffi::c_void;
use libc::{c_char, c_int, size_t};


fn meme_set_rust(ptr: *mut u8, c: u8, len: usize) -> *mut u8 {
    if ptr.is_null() { return ptr }

    for i in 0..len {
        unsafe {*(ptr.add(i)) = c as u8;}
    }

    ptr
}

extern "C" fn mem_set_c(ptr: *mut c_void, c: c_int, len: size_t) -> *mut c_void {
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meme_set() {
        let mut buffer = [65u8; 12];
        unsafe {
            meme_set_rust(buffer.as_mut_ptr(), 0, 12);
        }
        for i in 0..12 {
            println!("{}", buffer[i]);
        }
    }
}