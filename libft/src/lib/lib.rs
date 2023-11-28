// void *memset(void *b, int c, size_t len);

mod rust;
mod c;

use std::ffi::c_void;
use libc::{c_char, c_int, c_uchar, c_uint, size_t};


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meme_set() {
        let mut buffer = [65u8; 12];
        // unsafe {
        //     bzero_rust(buffer.as_mut_ptr(), 12);
        // }
        // for i in 0..12 {
        //     println!("{}", buffer[i]);
        // }
    }
}