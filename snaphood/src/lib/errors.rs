pub enum ErrorsTypes<'a> {
    NoArgs(u8),
    ArgsWithNoFlag(u8),
    UnknownFlag(u8, &'a str),
    FlagExpectArgs(u8, &'a str),
    FlagExpectNoArgs(u8, &'a str)
}

impl<'a> ErrorsTypes<'a> {
    pub fn print(&self) {
        match self {
            ErrorsTypes::ArgsWithNoFlag(code) => {
                println!("{code}: ArgsWithNoFlag")
            }
            ErrorsTypes::NoArgs(code) => {
                println!("{code}: NoArgs")
            }
            ErrorsTypes::UnknownFlag(code, flag) => {
                print!("{code}: UnknownFlag {}", flag)
            }
            ErrorsTypes::FlagExpectArgs(code, flag) => {
                println!("{code}: FlagExpectArgs {}", flag)
            }
            ErrorsTypes::FlagExpectNoArgs(code, flag) => {
                println!("{code}: FlagExpectNoArgs {}", flag)
            }
        }
    }
}