use std::ffi::c_void;
use std::fmt::Debug;
use libc::{c_char, c_int, size_t};
use libft_lib::*;

fn view_buffer(buffer: &[u8], expected: &[u8], function_name: &str) {
    println!("------------> Begin Test for {} <------------", function_name);
    println!(" -> buffer: {:?}", buffer);
    println!(" -> expect: {:?}", expected);

    println!("-------------> Finish Test for {} <--------------", function_name);
    println!("|-");

}

fn test<T: Debug>(t: T) {
    println!("{:?}", t);
}

struct Req {
    body: String,
}

struct Res {
    body: i32,
}

struct Athena;

impl Athena {
    pub fn get(&self, route: &str, handler: fn(Req, Res)) {
        handler(Req {body: "anas jaidi".to_owned()}, Res {body: 12})
    }

    fn new() -> Self {
        Self
    }
}


fn main() {
    // let app = Athena::new();

    // let mut i =12;
    //
    //
    //
    // let mut x = &mut i;
    //
    // let   mut cb = move  || {
    //   *x = 13;
    // };
    //
    //
    // cb();
    //
    // println!("{}",i);

    let i = 12u32;
    let x = Some(x);

    match x {
        Some(x) if x == 12 => {},
        None => {}
    };

    match i {
        x @ 0..=10 => println!("0..=10 = {}", x),
        x @ 10..  if  x != 12 => println!("more {}", x),
        _ => println!("12")
    };
}

fn old_main() {



    let mut buffer: [u8; 12] = [97; 12];
    let mut dist: [u8; 12] = [102; 12];

    let void_ptr = buffer.as_mut_ptr() as *mut c_void;
    let ptr = buffer.as_mut_ptr();

    let mut dist_ptr = dist.as_mut_ptr();
    let mut dist_void = dist.as_mut_ptr() as * mut c_void;

    let c_size = 12 as size_t;
    let rust_size = 12usize;

    let _char_c = 'a' as c_char;
    let _char_rust = 'a';

    let num_c = 65 as c_int;
    let num_rust = 65i32;

    let i = "123".parse::<u8>().unwrap();


    // start c testing
    {

        c::mem_set_c(void_ptr, num_c, c_size);
        view_buffer(&buffer, &[65; 12], "memeset(c)");
        c::memcpy_c(void_ptr, dist_void, c_size);
        view_buffer(&buffer, &dist, "memcpy(c)");
        c::bzero_c(void_ptr, c_size);
        view_buffer(&buffer, &[0; 12], "bzero(c)");
        c::memccpy_c(void_ptr, dist_void, num_c, c_size);
        view_buffer(&buffer, &dist, "memccpy(c)");
    }
    // start rust testing
    {
        rust::memccpy_rust(ptr, dist_ptr, num_rust, rust_size);
        view_buffer(&buffer, &dist, "memccpy(rust)");
        rust::memcpy_rust(ptr, dist_ptr, rust_size);
        view_buffer(&buffer, &dist, "memcpy(rust)");
        rust::bzero_rust(ptr, rust_size);
        view_buffer(&buffer, &[0; 12], "bzero(rust)");
        rust::meme_set_rust(ptr, num_rust, rust_size);
        view_buffer(&buffer, &[65; 12], "memeset(rust)");
    }
}