#[allow(unused)]

fn dangle_pointer() {
    let r;          // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;   // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
  // println!("r: {}", r);//          |
}    

fn main() {
  dangle_pointer();
}