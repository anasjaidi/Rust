#[derive(Debug, Clone, Copy)]
struct Test {
    num :  i32
}

fn foo(t: Test) {
    println!("{:?}", t)
}

fn main() {
    let mut t =  Test { num: 12 };
    foo(t);
    foo(t);
}