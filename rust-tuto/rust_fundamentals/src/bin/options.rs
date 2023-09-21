struct Student {
    name: String,
    grade: Option<i32>,
}

fn main() {
    let s1 = Student {name: "anas jaidi".to_owned(),  grade: Some(20)};

    match s1 {
        Student { name, grade :Some(value)} => {
            println!("user {} has a grade of {}", name, value);
        },
        Student { name, grade: None } => {
            println!("user {} has no grade", name);
        },
    }
}