fn some() -> Option<i32> {
    None
}

fn main() {
    let a = match some() {
        None => None,
        Some(num) => Some(num + 1),
    };
    println!("{:?}", a);
    println!("{:?}", a);

    let b = some().map(|n| n + 1).map(|n| n * 2);
    println!("{:?}", b);
}
