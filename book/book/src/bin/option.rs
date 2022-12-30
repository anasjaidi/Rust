#[allow(unused)]

/*
    enum Option<T> {
      Some(T),
      None,
    }
*/
fn main() {
  let x = Some(12);
  let x = Some("Anas jaidi");
  let x: Option<&str> = None;
  /* -------------------------------------- */
  let y1 = 5;
  let y2 = Some(7);
  let mut z: i32;
  // to work with OptionValue you need to extract the value by handling all possible casses
  match y2 {
      Some(x) => z = y1 + x,
      None => z= y1
  }
  println!("{}", z);
  // another methode
  z = y1 + y2.unwrap_or(0);
  println!("{}", z);
}