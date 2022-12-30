#[allow(unused)]
fn main() {
  let mut s1 = String::from("anas jaidi");
  let i = not_take_ownership(&mut s1);
  // s1 steal valid because s just borrow s1 value
  //////////////////////////////////////////////////
  // you can have only one mut ref or any number of imut
  let s2 = &s1;
  let s3 = &s1;
}

fn not_take_ownership(s: &mut String) -> usize{
  s.len()
}