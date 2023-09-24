use std::thread::sleep;
use std::time::{Duration, Instant, SystemTime};
fn main() {
    let now = Instant::now();

    let time = SystemTime::now();

    sleep(Duration::from_secs(1));

    println!("{}", now.elapsed().as_nanos());
    // println!("{}", now);
    // println!("{}");

}