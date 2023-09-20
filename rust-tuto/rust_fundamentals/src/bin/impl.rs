struct Temp {
    number: i32,
}
impl Temp {
    fn print(&self) {
        println!("{}", self.number);
    }
}
struct Temp_2 {
    number: i32,
}
impl Temp_2 {
    fn print(tmp: &Temp_2) {
        println!("{}", tmp.number);
    }
}

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

struct Temp_3 {
    number: i32,
    color: Color,
}
impl Temp_3 {
    fn print(tmp: &Temp_3) {
        println!("{}", tmp.number);
        tmp.color.print();
    }
    fn create() -> Self {
        Self { number: 14, color: Color::Blue }
    }
}

fn main() {
    let tmp = Temp { number: 12 };
    let tmp_2 = Temp_2 { number: 12 };
    let tmp_3 = Temp_3::create();
    tmp.print();
    Temp_2::print(&tmp_2);
    Temp_3::print(&tmp_3);
}
