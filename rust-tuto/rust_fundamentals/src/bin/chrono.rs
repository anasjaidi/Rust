use chrono::*;

fn main() {
    let now = Local::now();

    println!("{}", now.format("%h %m %s"))
}