#[derive(Debug, Clone, Copy)]
struct Test {
    num :  i32
}

fn foo(t: Test) {
    println!("{:?}", t.num)
}
 
fn main() {
    let t =  Test { num: 12 };
    foo(t);
    foo(t);
}