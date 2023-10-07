pub enum ErrorsTypes {
    NoArgs(u8),
    ExpectFolder(u8, String),
    ArgsWithNoFlag(u8),
    UnknownFlag(u8, String),
    FlagExpectArgs(u8, String),
    FlagExpectNoArgs(u8, String),
    FileReadPermissionDenied(u8),
    ExpectFile(u8, String),
    FileNotFound(u8, String),
    FolderNotFound(u8, String),
    FlagIsRequired(u8, String),
}

impl ErrorsTypes {
    pub fn print(&self) {
        match self {
            ErrorsTypes::FolderNotFound(_code, folder) => {
                println!("folder not found {}", folder)
            }
            ErrorsTypes::ExpectFolder(_code, folder) => {
                println!("expect folder: {}", folder)
            }
            ErrorsTypes::FlagIsRequired(_code, f) => {
                println!("flag is required {f}")
            }

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
            }
            Self::FileReadPermissionDenied(_code) => {
                println!("file not have read permission")
            }
            Self::ExpectFile(_code, file) => {
                println!("expect file: {}", file)
            }
            Self::FileNotFound(_code, file) => {
                println!("file not found: {file}")
            }
        }
    }
}
