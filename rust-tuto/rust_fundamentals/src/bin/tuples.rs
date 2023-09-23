#[allow(unused)]
enum EmployeeAccess {
    Full,
    Hybrid
}

fn one_two_three() -> (i8, i8, i8){
    return (1, 2, 3);
}
#[allow(unused)]
fn main() {
    let nums = one_two_three();
    let  (x, y, z)= one_two_three();
    println!("{:?} == {:?}", nums.0, x);
    println!("{:?} == {:?}", nums.1, y);
    println!("{:?} == {:?}", nums.2, z);

    let employee_1 = ("anas jaidi", EmployeeAccess::Hybrid);

    let (employee_name, employee_access) = employee_1;

    println!("{:?}", employee_1.0);
}