
#[allow(unused)]
fn main() {
  let hw = String::from("Hello, World!");
  let hws = &hw[..];
  let h = "Hello world"; // both hello and h are string slice &str because they point directly into memory
  let arr = [1,1,1];
  let arrs = &arr[..2];
  let hw = "Hello, World!";
  let hello = first_world(hw);
  


  
}

fn first_world(s: &str) -> &str {
  let bytes: &[u8] = s.as_bytes();
  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
       return &s[..i];
    }
  }
  &s[..] // return a slice a pointer to a portion of actual data
}

// fn first_world(s: &String) -> usize {
//   let bytes: &[u8] = s.as_bytes();
//   for (i: usize, &item: u8) in bytes.iter().enumerate() {
//     if item == b' ' {
//       return i;
//     }
//   }
// s.1en()
// }