fn main() {
    let mut a = Some(12);
    while let Some(num) = a {
        if a == None {
            break;
        } else {
            a = Some(num + 1);
        }
    }

    let mut v = vec![1, 2, 3];
    let mut it = v.iter();

    while let Some(n)  = it.next() {
        println!("{n}")
    }
}
