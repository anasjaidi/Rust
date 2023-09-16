struct MyStruct {
    field_a: i32,
    field_b: f32
}

fn main() {
    const A: MyStruct = MyStruct {
        field_a: 1,
        field_b:  1.00,
    };
    println!("{:?}", A.field_a);
    println!("{:?}", A.field_b);
}