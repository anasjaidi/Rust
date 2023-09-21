use std::collections::HashMap;

fn main() {
    let mut hash: HashMap<isize, String> = HashMap::new();

    hash.insert(1, "anas jaidi".to_owned());

    let i : isize = 1;
    match hash.get_mut(&i) {
        None => println!("no one found with this id in the db"),
        Some(value) => {
            println!("name is {}", value);
        }
    }

    for t in hash.iter() {
        println!("{} {}", t.0, t.1)
    }

    for key in hash.keys() {
        println!("{}", key)
    }

    for v in hash.values() {
        println!("{}", v)
    }
}