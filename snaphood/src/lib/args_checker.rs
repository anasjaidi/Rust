use std::{
    collections::HashMap,
    fs::{self, Metadata},
    os::unix::prelude::PermissionsExt,
};

use crate::{data::Flag, errors::ErrorsTypes};

fn check_file(metadata: &Metadata, mode: u32) -> Result<(), ErrorsTypes> {
    if !metadata.is_file() {
        return Err(ErrorsTypes::ExpectFile(12));
    }

    if metadata.permissions().mode() & mode == 0 {
        return Err(ErrorsTypes::FileReadPermissionDenied(12));
    }
    Ok(())
}

pub fn check_args(map: & HashMap<Flag, Vec<String>>) -> Result<(), ErrorsTypes> {
    for (k, v) in map {
        match k.args_type {
            crate::data::ArgsType::ReadFile => {
                for file in v {
                    if let Ok(f) = fs::metadata(file) {
                        match check_file(&f, 0b100) {
                            Err(err) => return Err(err),
                            Ok(_) => return Ok(()),
                        }
                    }
                }
            }
            _ => {}
        }
    }
    return Ok(())
}
