use std::{
    collections::HashMap,
    fs::{self, Metadata},
    os::unix::prelude::PermissionsExt,
};

use crate::{
    data::{ArgsType, Flag},
    errors::ErrorsTypes,
};

fn check_file(metadata: &Metadata, mode: u32) -> Result<(), ErrorsTypes> {
    if !metadata.is_file() {
        return Err(ErrorsTypes::ExpectFile(12));
    }

    if metadata.permissions().mode() & mode == 0 {
        return Err(ErrorsTypes::FileReadPermissionDenied(12));
    }
    Ok(())
}

fn check_flag_arg_content(f: &Flag, args: &Vec<String>) -> Result<(), ErrorsTypes> {
    match f.args_type {
        ArgsType::NoArgs if args.len() != 0 => Err(ErrorsTypes::FlagExpectNoArgs(
            12,
            f.flag_long_form.to_owned(),
        )),
        _ => {
            if args.len() > 0 {
                Ok(())
            } else {
                Err(ErrorsTypes::FlagExpectArgs(12, f.flag_long_form.to_owned()))
            }
        }
    }
}

pub fn check_args(map: &HashMap<Flag, Vec<String>>) -> Result<(), ErrorsTypes> {
    for (k, v) in map {
        check_flag_arg_content(k, v)?;
        match k.args_type {
            ArgsType::ReadFile => {
                for file in v {
                    if let Ok(f) = fs::metadata(file) {
                        match check_file(&f, 0b100) {
                            Err(err) => return Err(err),
                            Ok(_) => return Ok(()),
                        }
                    } else {
                        return Err(ErrorsTypes::FileNotFound(12, file.to_owned()));
                    }
                }
            }
            _ => {}
        }
    }
    return Ok(());
}
