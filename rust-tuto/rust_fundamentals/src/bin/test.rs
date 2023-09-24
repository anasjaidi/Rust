fn caps(str: &str) -> String {
    str.to_uppercase()
}

fn main() {
    
}

#[cfg(test)]
mod tst {
    use crate::caps;
    #[test]
    fn test_caps() {
        assert_eq!(caps("anas"), String::from("ANAS"), "is not work");
    }
}