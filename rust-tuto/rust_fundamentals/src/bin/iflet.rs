fn main() {
    let a = Some(12);

    if let Some(num) = a {
        dbg!(num);
    }
}