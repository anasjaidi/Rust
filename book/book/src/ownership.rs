// 1). Each value in rust has a variable that's called it's owner
// 2). the can be only one owner at the time
// 3). when the owner go out of scope data deatached

fn main() {
  {
    let x = String::from("anas jaidi");
  } // x deatached 
  let x =1;
  let y = x; // copy

  let s1 = String::from("anas");
  // let s2 = s1; // move not shallow
  // s1 is not valid anymore because owner ship od the data is moved to s2
  // instaed
  let s2 = s1.clone();
  // this methode copy the data and both s1 s2 are valids
  take_ownership(s2);
  // s2 no more valid the take_ownership takes the ownership so not possible to use data after move
  let s3 = give_ownership(); // s3 = "str"
  makes_copy(y); // y steal valid 
  takes_and_give_back(s1); // s1 is steal valid owner has moved and back by return
}

// return move the ownership of the returned values
fn give_ownership() -> String {
  let str = String::from("str");
  str
}

fn take_ownership(s: String) {
  println("{}", s);
}

// cause u8 are a scalar tyoe rust do a deep copy instead
fn makes_copy(i: u8) {
  println!("{}", i);
}

fn takes_and_give_back(s: String) -> String{
  s
}