struct Temp {
    number: i32,
}
impl Temp {
    fn print(&self) {
        println!("{}", self.number);
    }
}
struct Temp2 {
    number: i32,
}
impl Temp2 {
    fn print(tmp: &Temp2) {
        println!("{}", tmp.number);
    }
}

#[allow(unused)]
enum Color {
    Red,
    Blue
}

impl Color {
    fn print(&self) {
        match self {
            Color::Blue => println!("blue"),
            Color::Red => println!("red"),
        }
    }
}

struct Temp3 {
    number: i32,
    color: Color,
}
impl Temp3 {
    fn print(tmp: &Temp3) {
        println!("{}", tmp.number);
        tmp.color.print();
    }
    fn create() -> Self {
        Self { number: 14, color: Color::Blue }
    }
}

fn main() {
    let tmp = Temp { number: 12 };
    let tmp_2 = Temp2 { number: 12 };
    let tmp_3 = Temp3::create();
    tmp.print();
    Temp2::print(&tmp_2);
    Temp3::print(&tmp_3);
}
