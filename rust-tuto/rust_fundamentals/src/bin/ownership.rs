enum Test {
    V1,
    V2
}

fn bar(test: & mut Test) {
    match test {
        _ => test,
    };
}

fn foo(mut i: i8) {
    i = i + 1;
}

fn main() {
    let i = 1;
    foo(i);
    foo(i);

    let mut test = Test::V1;
    bar(&mut test);
    bar(&mut test);
}