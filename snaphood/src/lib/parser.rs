use std::collections::HashMap;

use crate::{
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
    flag: &str,
    map: &mut HashMap<String, Vec<String>>,
    mut index: usize,
    input: &Vec<String>,
) -> usize {
    while index < input.len() && !input[index].starts_with("-") {
        if let Some(entry) = map.get_mut(flag) {
            entry.push(input[index].clone());
        } else {
            map.insert(flag.to_owned(), vec![input[index].clone()]);
        }
        index += 1;
    }

    index
}

pub fn parse_input<'a>(
    input: &'a Vec<String>,
    allowed_flags: &Vec<Flag>,
) -> Result<HashMap<String, Vec<String>>, ErrorsTypes<'a>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut self_flag = None;

    let mut i = 0 as usize;

    if input.len() == 0 {
        return Err(ErrorsTypes::NoArgs(1));
    }

    while i < input.len() {
        if !input[i].starts_with("-") {
            return match self_flag {
                Some(f) => Err(ErrorsTypes::FlagExpectNoArgs(12, f)),
                None => Err(ErrorsTypes::ArgsWithNoFlag(12)),
            };
        }
        match check_flag(&input[i], allowed_flags) {
            Some(flag) => match flag.flag_type {
                FlagType::ContainerFlag => {
                    let index = get_flag_input(flag.flag_long_form, &mut map, i + 1, input);

                    if index == i + 1 {
                        return Err(ErrorsTypes::FlagExpectArgs(12, &input[i]));
                    }

                    i = index;
                }
                FlagType::SelfFlag => {
                    map.insert(flag.flag_long_form.to_owned(), vec![]);
                    self_flag = Some(&input[i]);
                    i += 1
                }
            },
            None => {
                return Err(ErrorsTypes::UnknownFlag(12, &input[i]));
            }
        }
    }

    Ok(map)
}
