use std::process;

fn main() {
  println!("pid: {}", process::id());
  ctrlc::set_handler(move || {
    println!("handled")
  });
  loop {
      
  }
}