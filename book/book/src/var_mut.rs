fn main() {
  // vars in rust are immutable by default
  let x = 5;
  // if you want mutable one use mut keyword in the front
  let mut y = 12;
  y = 30;
  // const must be uppercase value availble at compile time and typed
  const Z_CONST : u32 = 1_000; // syntatic suger _
  // shadowing move the ownership to the redeclared one 
  // until is end or redclare other one 
  // you can change mut and type
  let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
    }

    println!("The value of x is: {x}"); // 6
}