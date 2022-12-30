#[allow(unused)]

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
fn main() {
    let c = Coin::Dime;

    let c_in_cents = value_in_cents(&c);
    println!("{}", c_in_cents);

    let c_in_cents_plus_one = plus_one(Some(c_in_cents));
    println!("{}", c_in_cents_plus_one.unwrap_or(1));
    let c_in_cents_plus_one = plus_one(None);
    println!("{}", c_in_cents_plus_one.unwrap_or(22222222));
}

fn value_in_cents(c: &Coin) -> i32{
    match c {
        Coin::Dime => 1,
        Coin::Nickel => 5,
        Coin::Penny => 10,
        Coin::Quarter => 25,
        // _ => {
        //     println!("Other");
        //     0
        // } // default operator,
    }
}


fn plus_one(x:Option<i32>) -> Option<i32> {
  match x {
      Some(val) => Some(val + 1),
      _ => None,
  }
}