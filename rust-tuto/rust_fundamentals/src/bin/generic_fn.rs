//////////////////////////////
/// syntax
fn swap<T>(a: &mut T, b: &mut T) {}

fn fn1<T: Copy + Clone, U: Copy>(a: T, b: U) {}

fn fn2<T, U>(a: T, b: U)
where
    T: Copy + Clone,
    U: Copy,
{
}

//////////////////////////
/// seatup code

struct Snack;

impl Move for Snack {
    fn move_to(&self, x: i32, y: i32) {
        println!("slide to x: {} - y: {}", x, y);
    }
}

struct Car;

impl Move for Car {
    fn move_to(&self, x: i32, y: i32) {
        println!("drive to x: {} - y: {}", x, y);
    }
}
trait Move {
    fn move_to(&self, x: i32, y: i32);
}

//////////////////

fn mover_a(a: &impl Move, x: i32, y: i32) {
    a.move_to(x, y);
    ()
}

fn mover_b<T: Move>(a: &T, x: i32, y: i32) {
    a.move_to(x, y);
    ()
}

fn mover_c<T>(a: &T, x: i32, y: i32)
where
    T: Move,
{
    a.move_to(x, y);
    ()
}

fn main() {}
