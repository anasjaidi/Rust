pub fn meme_set_rust(ptr: *mut u8, c: i32, len: usize) -> *mut u8 {
    if ptr.is_null() { return ptr }

    for i in 0..len {
        unsafe {*(ptr.add(i)) = c as u8;}
    }

    ptr
}