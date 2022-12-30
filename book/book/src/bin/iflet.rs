fn main() {
    let five = Some(5);

    let num  = if let Some(x) = five {
      x
    } else {
      -1
    };
    println!("{}", num)
}