use std::fs::File;
use std::io::Read;
use std::ops::Add;
use std::os::fd::{AsRawFd, FromRawFd};

static mut REMAIN: String = String::new();

const BUFFER_SIZE: u32 = 1;

fn get_line(buffer: &[u8]) -> Option<String> {
    let mut line = String::new();
    for (i, &b) in buffer.iter().enumerate() {
        if b == b'\n' {
            // line.push(b as char);
            unsafe {
                if !REMAIN.is_empty() {
                    line.push_str(&REMAIN);
                    REMAIN.clear()
                }
                if i < BUFFER_SIZE as usize - 1 {
                    REMAIN = String::from_utf8_lossy(&buffer[i..]).into_owned();
                }
                return Some(line);
            }
            break;
        }
        line.push(b as char);
    }
    unsafe {
        REMAIN.push_str(&line);
    }
    None
}

fn get_next_line(fd: usize) -> String {
    let mut file: File = unsafe { FromRawFd::from_raw_fd(fd as i32) };
    let mut buffer = [0u8; BUFFER_SIZE as usize];
    loop {
        let size = file.read(&mut buffer).unwrap();
        print!("{:?}", get_line(&buffer));
        if size == 0 {
            break;
        }
    }

    String::new()
}

fn main() {
    let file = File::open("./a.txt").unwrap();
    let fd = file.as_raw_fd();
    println!("{}", fd);
    get_next_line(fd as usize);
}
