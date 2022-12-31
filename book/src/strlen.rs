#[allow(unused)]

  pub fn strlen(s: &String) -> usize{
      s.len()
  }

pub fn test() {
  let len = strlen(&String::from("anas jaidi"));
  println!("len: {}", len)
}