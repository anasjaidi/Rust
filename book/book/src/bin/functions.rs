#[allow(unused)]
fn main() {
  let greeting = greet("Alice");
  let (sum, difference) = add(3, 4);
  // statement vs expresion
  // statement does'nt return
  // let a = (let b = 1);
  // expresion return
  let a = sum + difference;
}

// you can create the function by fn keyword 
// you can add params by put thire name: type, 
// return by writeen by -> type
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
fn add(x: i32, y: i32) -> (i32, i32) {
    (x + y, x - y) // if the last line is the return downt wrie return or ;
}
