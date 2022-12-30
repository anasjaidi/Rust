#[allow(unused)]
fn main() {
    // rust has to major types scalar and coumpund
    //  scalar
    // bool: A boolean type that can hold the values true or false.
    let b = true;
    let b = false;
    // char: A character type that represents a single Unicode scalar value.
    let mut c = 'c';
    // i8, i16, i32, i64, isize: Integer types with varying sizes. The i prefix stands for "signed".
    let i = 12_000; // dec
    let i = 0xff; // hex
    let i = 0o77; // octal
    let i = 0b1_00000000; // bin
    let i: u8 = b'a'; // byte obly u8
                      // u8, u16, u32, u64, usize: Integer types with varying sizes. The u prefix stands for "unsigned".
    let u: u8 = 0;
    // f32, f64: Floating point types with single and double precision, respectively.
    let mut f = 1.0;
    //////////////////////////////////////////////////////////////////////////////////////////////////////////
    //////////////////////////////////////////////////////////////////////////////////////////////////////////
    // compund
    // tuple: A fixed-size collection of values with different types.
    let mut t: (i8, char, bool) = (1, 'c', true);

    let  (mut a,b,c) = t;

    a = 2;
    let n = t.0;
    // array: A fixed-size array type, where the size is specified as part of the type.
    let arr : [u8; 3] = [1,2,4];
    
    println!("{}", arr[1]);

    for i in arr.iter() {
        println!("{}", i);
    }
}
