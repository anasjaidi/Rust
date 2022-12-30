#[allow(unused)]
#[derive(Debug)]
enum IpAddrKind {
  V4(u8, u8, u8, u8),
  V6(String)
}

#[allow(unused)]
enum Message {
    Quit,
    Move {x:i32, y:i32},
    Write(String),
    ChangeColor(u8, u8, u8)
}

impl Message {
  fn function() {
    println!("Message function");
  }
}

#[allow(unused)]
fn main() {
    let localhost = IpAddrKind::V4(127, 0, 0, 1);
    println!("{:#?}", localhost);
  Message::function();
}