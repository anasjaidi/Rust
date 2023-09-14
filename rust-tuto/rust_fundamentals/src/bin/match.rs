fn main() {
    /*
        rust match is like the if control flow
        but with one key diff
        we need to cover all possibilities
    */
    let age = 20;
    match age {
        20 => println!("is 20"),
        30 => {
            println!("30");
        },
        _ => println!("yes"), // default
    }
}
