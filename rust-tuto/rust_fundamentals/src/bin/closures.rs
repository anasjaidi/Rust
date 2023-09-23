fn add_fn(a: i32, b: i32) -> i32 {
    a + b
}

fn operate(f: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    f(a, b)
}

fn main() {
    let add_regular = |a: i32, b: i32| -> i32 { a + b };

    let add = |a, b| a + b;

    println!("{:?}", add(1, 2));
    println!("{:?}", add_regular(1, 2));
    println!("{:?}", add_fn(1, 2));
    println!("{:?}", operate(add, 1, 2));
}
