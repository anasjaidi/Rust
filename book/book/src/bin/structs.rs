#[allow(unused)]
struct User {
  username: String,
  email: String,
  active:bool,
  sign_in_count: u64
}

#[allow(unused)]
struct Color(i32,i32,i32);

#[allow(unused)]
#[derive(Debug)]
struct Point {
  height: i32,
  width: i32,
}

#[allow(unused)]
impl Point {
  fn area(&self) -> i64{
    (self.height * self.width) as i64
  }
}


#[allow(unused)]
impl Point {
    fn squre(size:i32) -> Point {
      Point { height: size, width: size }
    }
}


#[allow(unused)]
fn main() {
  let mut user1 = User {
    username: String::from("anasjaidi"),
    email: String::from("anas.jaidi@icloud.com"),
    active: true,
    sign_in_count: 1
  };

  let name = user1.username;

  user1.active = false;

  let user2 = build_user(String::from("jaidi"), String::from("anas@jaidi.com"));

  let user3 = User {
    email: String::from("jaidi@jaidi1.com"),
    username: String::from("anas jaidi"),
    ..user2
  };

  let p = Point{height:100, width:50};
  println!("{:#?}", p);
  println!("{:#?}", p.area());

  
  let square = Point::squre(90);
  println!("{:#?}", square);
}


fn build_user(u: String, e: String) -> User {
  User {
    username:u,
    email:e,
    active:true,
    sign_in_count:1
  }
}