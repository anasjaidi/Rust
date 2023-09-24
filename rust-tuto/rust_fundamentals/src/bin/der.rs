#[derive(Debug)]

enum Status {
    Done
}

#[derive(Debug)]
struct Item {
    status: Status
}

fn main() {
    let a = Item {status: Status::Done};

    let variant = Status::Done;

    let variant_2 = variant; // move owner ship

    println!("{:?}", variant); // Error: value borrowed here after move

    let variant_3 = a.status;

    println!("{:?}", a);


}