

struct Test {
    num: i32,
}
impl Test {
    fn print(&self) {
        println!("{:?}", self.num);
    }
}


fn main() {
    let v1: Vec<i32> = vec![1, 3];
    let mut v2: Vec<i32> = Vec::new();
    let v3 = vec![
        Test {num: 1},
        Test {num: 2},
        Test {num: 3},
        Test {num: 4},
    ];
    v2.push(1);
    v2.push(3);
    println!("{:?}", v1[0]);
    println!("{:?}", v1);
    println!("{:?}", v2); 

    for num in v2 {
        println!("{}", num)
    }
    for num in &v3 {
        num.print()
    }
    println!("{}", v3.len());
}