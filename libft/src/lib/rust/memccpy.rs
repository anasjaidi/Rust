pub fn memccpy_rust(dst: *mut u8,src: *mut u8,c: i32, len: usize) -> *mut u8 {
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