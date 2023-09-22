fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

fn operate(F: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    F(a, b)
}

fn main() {
    let add = |a: i32, b: i32| -> i32 { a + b };

    let add = |a, b| a + b;

    println!("{:?}", add(1, 2));
    println!("{:?}", add_fn(1, 2));
    println!("{:?}", operate(add, 1, 2));
}
