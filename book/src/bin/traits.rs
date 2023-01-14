#![allow(unused)]
use std::fmt::{format, Display, Debug};

#[derive(Debug)]
struct Tweet {
  username: String,
  content: String,
  reply: String,
  retweet: String
}

#[derive(Debug)]
struct NewsArticle {
  author: String,
  headline: String,
  content: String,
}


impl Summary for Tweet {
  fn summrise_author(& self) -> String {
      format!("{}", self.username)
  }
  fn summrise(&self) -> String {
    format!("{} {}", self.username , self.content)
  }
}

impl Summary for NewsArticle {
  fn summrise_author(& self) -> String {
      format!("{}", self.author)
  }
  // fn summrise(&self) -> String {
  //     format!("{} {}", self.author , self.content)
  // }
}

fn notify<T: Summary + Debug>(obj1: &T, obj2: &T) -> String {
  obj1.summrise()
}

fn notify2<T: Summary + Debug, U: Summary + Debug>(obj1: &T, obj2: &U) -> String {
  obj1.summrise()
}

fn notify3<T, U>(obj1: &T, obj2: &U) -> String
  where T: Summary + Debug,
        U: Summary + Debug {
  obj1.summrise()
}

fn notify1(ob1j: &(impl Summary + Debug), obj2: &(impl Summary + Debug)) -> String {
  ob1j.summrise()
}


trait Summary {
  fn summrise_author(& self) -> String;
  fn summrise(&self) -> String {
    format!("Read more! from {}.", self.summrise_author())
  }
}

fn main() { // shared behaviour
  let mut t = Tweet { username :String::from("anasjaidi"), content: String::from("just another tweet"), reply: String::new(), retweet: String::new() };
  let mut a=  NewsArticle { author: String::from("anas jaidi"), headline: String::from("just title"), content: String::from("just other content") };

  println!("{}", t.summrise());
  println!("{}", a.summrise());
  println!("{}", notify(&t, &t));
  println!("{}", notify1(&a, &t));
  println!("{}", notify2(&a, &t));
  println!("{}", notify3(&t, &t));
}