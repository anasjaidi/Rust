fn main() {
    let v1: Vec<i32> = vec![1, 2, 3, 4].iter().map(|num| *num + 1).collect();
    let mut v2 = Vec::new();

    for num in  &v1 {
        v2.push(*num + 1);
    }

    println!("{:?}", v2);

    let mut v3: Vec<i32> = v2.iter().map(|num| num + 1).collect();
    println!("{:?}", v3);


    let num_three: &i32 = v2.iter().find(|num| **num == 3).unwrap();
    println!("{:?}", num_three);

    let a: &mut i32 = v3.iter_mut().max().unwrap();
    println!("{a}");
    *a += 1;
    println!("{}", v3.iter().max().unwrap());
    let b:Option<&i32> = v3.iter().min();


    let data: Vec<_> = vec![1, 2, 3].iter().map(|n| n * 3).filter(|n| n >= &6).collect();
}