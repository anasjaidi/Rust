use std::ops::{DerefMut, Deref};

struct Bucket(i32);

impl Deref for Bucket {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bucket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn hoo(a: &i32) {
    println!("{}", a)
}

impl Drop for Bucket {
    fn drop(&mut self) {
        println!("dropping bucket")
    }
}

fn main() {
    let i = Bucket(12);

    drop(i);

}
