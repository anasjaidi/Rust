use my_lib::parser as prs;

mod exec;

fn main() {
    println!("Hello, world!");
    prs::p(); // prs is alias for parser
    exec::anasjaidi();
}
