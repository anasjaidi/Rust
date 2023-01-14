use std::fs::{self, File};
use std::io::{self, ErrorKind, Read, Error};

#[allow(unused)]
fn get() -> Result<String, io::Error>{
    let f = fs::read_to_string("src/bin/errors.rs");
    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match fs::File::create("./tt.c") {
                Ok(f) => "anas".to_string(),
                Err(e) => {return Err(e);},
            },
            _ => panic!(""),
        },
    };



    let f = fs::File::open("ttt.c").unwrap_or_else(|err| if err.kind() == ErrorKind::NotFound {
        fs::File::create("ttt.c").unwrap_or_else(|err| {
            panic!();
        })}
        else {
            panic!("");
        });
    

    let f = fs::File::open("ttt.c").unwrap();
    let f = fs::File::open("ttt.c").expect("error message");
    let f = fs::File::open("ttt.c")?; // return the error


    Ok("anas".to_string())
}


fn a() {
    b();
}

fn b() {
    c();
}

fn c() {
    panic!("abort") // `RUST_BACKTRACE=1` at the front of cargo comand
}

fn read_user(path: &str) -> Result<String, Error>{
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}

fn read_pass(path: &str) -> Result<String, Error>{
    fs::read_to_string(path)
}

fn main() {
    // a();
    let s = read_user("src/main.rs").unwrap();
    let p = read_pass("src/main.rs").unwrap();

    println!("{} \n\n\n\n\n\n\n\n {}", s, p);
}