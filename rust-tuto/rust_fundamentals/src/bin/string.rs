fn print_text(str: &str) {
    print!("{}", str);
}

struct NOT_USE_STRING_SLICE_IN_STRUCT {
    str: String,
}

fn main() {
    let str = "anas jaidi".to_owned();
    let s = String::from("anas jaidi");

    let v = vec![NOT_USE_STRING_SLICE_IN_STRUCT {
        str: String::from("dont use it"),
    }];

    print_text(&str);
    print_text(&s);
    print_text("anas jaidi");
}
