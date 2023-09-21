struct A {
    price: i32,
    name: String,
}

fn main() {
    let a = A {price: 12, name: "anas jaidi".to_owned()};

    match a {
        A { price: 13, ..} => {
            println!("13");
        },
        A { price: other, name: name_other} => {
            println!("name is {}", name_other);
            println!("price is {}", other);
        },
    }
}