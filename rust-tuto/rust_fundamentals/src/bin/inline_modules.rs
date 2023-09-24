#![allow(unused)]
use std::collections::HashMap; // work for outer scope (file)

mod greet {
    use std::collections::HashMap; // work only for this innwe scop (this mod like a file)
    pub fn hi() {
        let hash: HashMap<i32, i32> = HashMap::new();
        println!("hi")
    }

    pub fn bye() {
        println!("bye")
    }
}

mod math {

    pub fn mult(a: i32, b: i32) -> i32 {
        a * b
    }

    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}

fn main() {
    use greet::*;
    hi();
    bye();
    use math::{add, sub};
    add(1, 2);
    add(1, 2);
    math::mult(1, 2);
}
