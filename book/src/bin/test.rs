#[allow(unused_mut)]
fn main() {
  let mut v: Vec<u8> = Vec::new();
  v.reserve(100);
  v.reserve_exact(200);
  println!("capacity: {}", v.capacity());
  println!("length: {}", v.len());
}