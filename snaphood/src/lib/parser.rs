use std::collections::HashMap;

use crate::{
    args_checker,
    data::{Flag, FlagType},
    errors::ErrorsTypes,
};

fn check_flag<'a>(flag: &str, flags: &'a Vec<Flag>) -> Option<&'a Flag> {
    for f in flags {
        if f.flag_long_form.eq(flag) || f.flag_short_form.eq(flag) {
            return Some(f);
        }
    }
    None
}

fn get_flag_input(
    flag: &Flag,
    map: &mut HashMap<Flag, Vec<String>>,
    mut index: usize,
    input: &Vec<String>,
) -> usize {
    while index < input.len() && !input[index].starts_with("-") {
        if let Some(entry) = map.get_mut(flag) {
            entry.push(input[index].clone());
        }
        index += 1;
    }

    index
}

pub fn parse_input(
    input: &Vec<String>,
    allowed_flags: &Vec<Flag>,
) -> Result<HashMap<Flag, Vec<String>>, ErrorsTypes> {
    let mut map: HashMap<Flag, Vec<String>> = HashMap::new();

    let mut i = 0 as usize;

    if input.len() == 0 {
        return Err(ErrorsTypes::NoArgs(1));
    }

    while i < input.len() {
        match check_flag(&input[i], allowed_flags) {
            Some(flag) => {
                map.insert(flag.clone(), vec![]);
                i += 1;
                i = get_flag_input(flag, &mut map, i, input);
            }
            None => {
                return Err(ErrorsTypes::UnknownFlag(12, input[i].clone()));
            }
        }
    }
    match args_checker::check_args(&map) {
        Err(err) => return Err(err),
        Ok(_) => return Ok(map),
    }
}
