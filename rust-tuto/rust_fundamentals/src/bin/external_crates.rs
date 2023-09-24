use humantime::{format_duration};
use std::time::Duration;
fn main() {
    let d = Duration::from_secs(12000);
    println!("{}", format_duration(d))
}