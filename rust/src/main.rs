#[derive(Debug)]
enum Test {
    X (i32),
}
impl Test {
  fn t(self: &Self) -> &i32
  {
    match self {
      Test::X(n) => return n,
    }
  }
}

fn main() {
    let s = Test::X(12);
    println!("{:?}", s.t());
}
