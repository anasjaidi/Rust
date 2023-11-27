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

fn bzero_rust(ptr: *mut u8, len: usize) -> *mut u8 {
    if ptr.is_null() {
        return ptr
    }
    for i in 0..len {
        unsafe {
            *(ptr.add(i)) = 0;
        }
    }
    ptr
}

extern "C" fn bzero_c(ptr: *mut c_void, len: size_t) -> *mut c_void {
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meme_set() {
        let mut buffer = [65u8; 12];
        unsafe {
            bzero_rust(buffer.as_mut_ptr(), 12);
        }
        for i in 0..12 {
            println!("{}", buffer[i]);
        }
    }
}