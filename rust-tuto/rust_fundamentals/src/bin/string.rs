fn print_text(str: &str) {
    print!("{}", str);
}

#[allow(unused)]
struct NotUseStringSliceInStructs {
    str: String,
}

#[allow(unused)]
fn main() {
    let str = "anas jaidi".to_owned();
    let s = String::from("anas jaidi");

    let v = vec![NotUseStringSliceInStructs {
        str: String::from("dont use it"),
    }];

    print_text(&str);
    print_text(&s);
    print_text("anas jaidi");
}
