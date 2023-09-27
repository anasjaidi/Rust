struct Value {
    value: i32
}

fn main() {
    let value = Value {value: 12};

    let value_ptr = Box::new(value);

    let value_stack = *value_ptr;
}