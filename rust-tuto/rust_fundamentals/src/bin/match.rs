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
    let age = 25;

    match age {
        n if n > 20 => {
            println!("Age is greater than 20: {}", n);
            // Additional code for age greater than 20
        }
        n if n < 20 => {
            println!("Age is less than 20: {}", n);
            // Additional code for age less than 20
        }
        _ => {
            println!("Age is equal to 20");
            // Additional code for age equal to 20 or other cases
        }
    }
}
