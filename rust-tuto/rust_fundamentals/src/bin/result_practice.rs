struct Student {
    name: String,
}

impl Student {
    fn new(age: i32, name: String) -> Result<Self, String> {
        match age {
            legal if legal >= 21 => Ok(Self { name }),
            _ => Err("under age".to_owned()),
        }
    }
}

fn main() {
    let s1 = Student::new(20, "anas jaidi".to_owned());
     match s1 {
        Err(err) => {println!("{}", err);},
        Ok(student) => {
            println!("{}", student.name)
        },
    };
}
