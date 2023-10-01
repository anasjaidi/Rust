pub struct StrSplit<'a> {
    data: &'a str
}

impl<'a> StrSplit<'a> {
    pub fn new(haystack: &str, del: &str) -> Self {
        StrSplit { data: "anas"}       
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}