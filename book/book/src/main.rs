use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use colored::*;


fn main() {
  println!("give a number:");
  let mut thread = thread_rng();
  let secret_key = thread.gen_range(1..=100);
  
  loop {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("failed");
  let guess : u32 = match guess.trim().parse() {
    Ok(i)=> i,
    Err(_) => continue,
  };
  match guess.cmp(&secret_key) {
    Ordering::Greater => {
      println!("{}","to big".red());
    },
    Ordering::Less => {
      println!("{}","to low".red());
    },
    Ordering::Equal => {
      println!("{}", "you win".green());
      break;
    }
  };
  }
  
}