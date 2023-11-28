pub fn bzero_rust(ptr: *mut u8, len: usize) -> *mut u8 {
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