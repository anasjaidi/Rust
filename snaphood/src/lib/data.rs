#[derive(Eq, PartialEq, Hash, Clone)]
pub enum FlagType {
    ContainerFlag,
    // SingleContainerFlag,
    SelfFlag,
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub enum ArgsType {
    ReadFile,
    CreateFile,
    ReadFolder,
    CreateFolder,
    NoArgs,
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Flag {
    pub flag_short_form: &'static str,
    pub flag_long_form: &'static str,
    pub flag_type: FlagType,
    pub args_type: ArgsType,
}

pub fn get_allowed_flags() -> Vec<Flag> {
    return vec![
        Flag {
            flag_type: FlagType::ContainerFlag,
            flag_long_form: "--input",
            flag_short_form: "-i",
            args_type: ArgsType::ReadFile,
        },
        Flag {
            flag_long_form: "--output",
            flag_short_form: "-o",
            flag_type: FlagType::ContainerFlag,
            args_type: ArgsType::CreateFolder,
        },
        Flag {
            flag_long_form: "--replace",
            flag_short_form: "-r",
            flag_type: FlagType::SelfFlag,
            args_type: ArgsType::NoArgs,
        },
    ];
}
