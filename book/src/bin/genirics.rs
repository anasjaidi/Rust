use core::fmt::Display;

struct s<T, F> {
  a: T,
  b: F
}

impl<T: Copy, F> s<T, F> {
  fn mix<V, W>(&self , other : s<V, W>) -> s<T, W> {
    s {
      a: self.a,
      b: other.b,
    }
  }
}

struct test<T> {
  x: T,
  y: T
}

enum Option<T> {
  Some(T),
  None
}

// impl for all types
impl<U> test<U> {
  fn x(&self) -> &U{
    &self.x
  }
}
// only one type
impl test<i32> {
  fn y(&self) -> &i32{
    &self.x
  }
}

enum Result<T, E> {
    Ok(T),
    Err(E)
}

fn gen<T: Display >(a: &mut T) -> &mut T {
  println!("{}", a);
  a
}

fn main() {}