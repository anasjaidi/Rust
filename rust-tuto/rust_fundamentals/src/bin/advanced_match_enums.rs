enum Tickets {
    A(i32),
    B(String, i32),
    C(String, i32),
}

fn main() {
  let v = vec![
    Tickets::A(12),
    Tickets::B("anas jaidi".to_owned(), 13),
    Tickets::C("robin hood".to_owned(), 14),
  ];

  for ticket in v {
      match ticket {
          Tickets::A(price) => {
            println!("price of ticket @A is: {}", price);
          },
          Tickets::B(name, price) => {
            println!("price of ticket @B is: {}", price);
            println!("name of ticket @B is: {}", name);
          },
          Tickets::C(name, price) => {
            println!("price of ticket @C is: {}", price);
            println!("name of ticket @C is: {}", name);
          }
      }
  }
}