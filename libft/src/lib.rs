// void *memset(void *b, int c, size_t len);

use std::ffi::c_void;
use libc::{c_char, c_int, c_uchar, c_uint, size_t};


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

fn memcpy_rust(dst: *mut u8,src: *mut u8, len: usize) -> *mut u8 {
    if dst.is_null() || src.is_null() {
        return dst
    }
    for i in 0..len {
        unsafe {
            *(dst.add(i)) = *(src.add(i));
        }
    }
    dst
}

extern "C" fn memcpy_c(dst: *mut c_void,src: *mut c_void, len: size_t) -> *mut c_void {
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

fn memccpy_rust(dst: *mut u8,src: *mut u8,c: isize, len: usize) -> *mut u8 {
    if dst.is_null() || src.is_null() {
        return std::ptr::null_mut()
    }
    for i in 0..len {
        unsafe {
            if *(dst.add(i)) == (c as u8) {
                return dst.add(i + 1);
            }
            *(dst.add(i)) = *(src.add(i));
        }
    }
    std::ptr::null_mut()
}

extern "C" fn memccpy_c(dst: *mut c_void,src: *mut c_void,c: c_int, len: size_t) -> *mut c_void {
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