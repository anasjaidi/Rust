use std::{io::{self, Write}};


fn get_user_input(prompt: &str) -> io::Result<String> {
    let mut buffer = String::new();

    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut buffer)?;
    
    Ok(buffer.trim().to_owned())
}
fn main() {
    let _ = io::stdout().flush();
    let username = get_user_input("enter username: ").unwrap();
    let password = get_user_input("enter password: ").unwrap();

    println!("name {}", username);
    println!("pass {}", password);
}