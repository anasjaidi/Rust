#[allow(unused)]
fn main() {
    let arr = [1, 2, 3]; // stack allocated because of fixed size
    let mut vec: Vec<i32> = Vec::new(); // heap allocated no fixed size
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    let vecp = vec![1, 2, 3];
    let third = vec[2];
    println!("{}", third);
    vec.push(5);
    match vec.get_mut(2) {
        Some(i) => {*i += 1;println!("{}", i)},
        None => println!("error"),
    }
    for i in &mut vec {
        *i += 1;
        println!("{}", i);
    }
    
}
