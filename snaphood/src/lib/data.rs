pub enum FlagType {
    ContainerFlag,
    SelfFlag,
}

pub struct Flag {
    pub flag_short_form: &'static str,
    pub flag_long_form: &'static str,
    pub flag_type: FlagType,
}

pub fn get_allowed_flags() -> Vec<Flag> {
    return vec![
        Flag {
            flag_type: FlagType::ContainerFlag,
            flag_long_form: "--input",
            flag_short_form: "-i",
        },
        Flag {
            flag_long_form: "--output",
            flag_short_form: "-o",
            flag_type: FlagType::ContainerFlag,
        },
        Flag {
            flag_long_form: "--replace",
            flag_short_form: "-r",
            flag_type: FlagType::SelfFlag,
        },
    ];
}
