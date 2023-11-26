// void *memset(void *b, int c, size_t len);

fn meme_set(ptr: *mut u8, c: u8, len: usize) -> *mut u8 {

    if ptr.is_null() {return ptr}
    let mut bytes = unsafe {
        std::slice::from_raw_parts_mut( ptr, len)
    };
    for b in  bytes.iter_mut() {
        *b = c;
    }
    ptr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meme_set() {
        let buffer = Box::new(12);
        let ptr: *mut u8 = buffer.as_mut_ptr();

        meme_set(ptr, 65, 4);

        for i in 0..4 {
            let byte = unsafe { *(ptr.add(i)) };
            let ch = char::from(byte);

            println!("Byte as char: {:?}", ch);
        }
    }
}