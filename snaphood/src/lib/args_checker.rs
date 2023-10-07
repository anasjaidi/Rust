use std::{
    collections::HashMap,
    fs::{self, Metadata},
    os::unix::prelude::PermissionsExt,
};

use crate::{
    data::{ArgsType, Flag},
    errors::ErrorsTypes,
};

#[derive(PartialEq)]
enum ArgType {
    Folder,
    File,
}

fn check_arg_type(
    metadata: &Metadata,
    mode: u32,
    arg_type: ArgType,
    arg_name: &str,
) -> Result<(), ErrorsTypes> {
    if arg_type == ArgType::File && !metadata.is_file() {
        return Err(ErrorsTypes::ExpectFile(12, arg_name.to_owned()));
    } else if arg_type == ArgType::Folder && !metadata.is_dir() {
        return Err(ErrorsTypes::ExpectFolder(12, arg_name.to_owned()));
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

pub fn required_flags_checker(
    required_flags: &Vec<Flag>,
    flags_map: &HashMap<Flag, Vec<String>>,
) -> Result<(), ErrorsTypes> {
    for f in required_flags {
        if let None = flags_map
            .iter()
            .find(|(k, _)| k.flag_long_form == f.flag_long_form)
        {
            return Err(ErrorsTypes::FlagIsRequired(12, f.flag_long_form.to_owned()));
        }
    }

    Ok(())
}

pub fn check_args(map: &HashMap<Flag, Vec<String>>) -> Result<(), ErrorsTypes> {
    for (k, v) in map {
        check_flag_arg_content(k, v)?;
        match k.args_type {
            ArgsType::ReadFile => {
                for file in v {
                    if let Ok(f) = fs::metadata(file) {
                        match check_arg_type(&f, 0b100, ArgType::File, file) {
                            Err(err) => return Err(err),
                            _ => {}
                        }
                    } else {
                        return Err(ErrorsTypes::FileNotFound(12, file.to_owned()));
                    }
                }
            }
            ArgsType::ReadFolder => {
                for folder in v {
                    if let Ok(meta) = fs::metadata(folder) {
                        match check_arg_type(&meta, 0b100, ArgType::Folder, folder) {
                            Err(err) => return Err(err),
                            _ => {}
                        }
                    } else {
                        return Err(ErrorsTypes::FolderNotFound(12, folder.to_owned()));
                    }
                }
            }
            _ => {}
        }
    }
    return Ok(());
}
