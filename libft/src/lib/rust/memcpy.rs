pub fn memcpy_rust(dst: *mut u8,src: *mut u8, len: usize) -> *mut u8 {
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