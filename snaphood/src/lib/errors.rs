pub enum ErrorsTypes {
    NoArgs(u8),
    ArgsWithNoFlag(u8),
    UnknownFlag(u8, String),
    FlagExpectArgs(u8, String),
    FlagExpectNoArgs(u8, String),
    FileReadPermissionDenied(u8),
    ExpectFile(u8)
}

impl ErrorsTypes {
    pub fn print(&self) {
        match self {
            Self::ArgsWithNoFlag(code) => {
                println!("{code}: ArgsWithNoFlag")
            }
            Self::NoArgs(code) => {
                println!("{code}: NoArgs")
            }
            Self::UnknownFlag(code, flag) => {
                print!("{code}: UnknownFlag {}", flag)
            }
            Self::FlagExpectArgs(code, flag) => {
                println!("{code}: FlagExpectArgs {}", flag)
            }
            Self::FlagExpectNoArgs(code, flag) => {
                println!("{code}: FlagExpectNoArgs {}", flag)
            },
            Self::FileReadPermissionDenied(_code) => {
                println!("file not have read permission")
            },
            Self::ExpectFile(_code) => {
                println!("expect file")
            }
        }
    }
}