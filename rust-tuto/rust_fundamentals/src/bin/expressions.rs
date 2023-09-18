enum AccessType {
    Admin,
    Basic,
    Guest,
    Manager,
}

fn check_access(access_type: AccessType) -> bool{
    let user_have_access = match access_type {
        AccessType::Admin => true,
        AccessType::Manager => true,
        _ => false,
    };
    user_have_access
}

fn main() {
    let a = if 1 == 1 { 12 } else { 13 };

    println!("{:?}", a);

    let b = match 1 {
        1 => 10,
        2 => 20,
        _ => 30,
    };

    println!("{:?}", b);

    let user_access_type = AccessType::Admin;
    println!("{:?}", check_access(user_access_type));
}
