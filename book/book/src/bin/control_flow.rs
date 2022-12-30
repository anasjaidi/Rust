#[allow(unused)]
fn main() {
  if true == true {

  } if false < true {

  } else {

  }
  // condition only need to be a boolean if number doesn't work
  let _a = if true {5} else {54};
  ///////////////////////////////////////////////////////////////
  let mut i = 0;

  let three = loop {
    // adding x after break returnded
    if i == 3 { break 3;} else if i == 2 {continue;}

    println!("{}", true);

    i += 1;
  };

  while i < 4 {
    i += 1;
  }
  
  for i in 1..4 {
    println!("go")
  }
}