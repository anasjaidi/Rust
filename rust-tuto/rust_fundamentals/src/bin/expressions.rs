fn main() {
    let a = if 1 == 1 {
        12
    } else {
        13
    };

    println!("{:?}", a);

    let b = match 1 {
        1 => 10,
        2 => 20,
        _ => 30
    };

    println!("{:?}", b);
}