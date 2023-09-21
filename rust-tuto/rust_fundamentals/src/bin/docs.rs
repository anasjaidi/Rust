/// function to add two i32 and return them
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/*
    you can visit rust doc by typing 
    rustup doc
*/
fn main() {
    add(12, 13);
    let str = "Anas Jaidi".to_owned();

    println!("{}", str.to_uppercase());
    println!("{}", str.to_lowercase());
}