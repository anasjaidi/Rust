use std::collections::HashMap;

use crate::data::{Flag, FlagType};

fn check_flag<'a>(flag: &str, flags: &'a Vec<Flag>) -> Option<&'a Flag> {
    for f in flags {
        if f.flag_long_form == flag || f.flag_short_form == flag {
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
            dbg!(entry);
        } else {
            map.insert(flag.to_owned(), vec![input[index].clone()]);
        }
        index += 1;
    }

    index
}

pub fn parse_input(
    input: &Vec<String>,
    allowed_flags: &Vec<Flag>,
) -> Result<HashMap<String, Vec<String>>, String> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    let mut i = 0 as usize;

    if !input[0].starts_with("-") {
        return Err("Form Forbiden".to_owned());
    }

    while i < input.len() {
        match check_flag(&input[i], allowed_flags) {
            Some(flag) => match flag.flag_type {
                FlagType::ContainerFlag => {
                    i = get_flag_input(flag.flag_long_form, &mut map, i, input)
                }
                FlagType::SelfFlag => {
                    map.insert(flag.flag_long_form.to_owned(), vec![]);
                    i += 1
                }
            },
            None => {
                return Err("Flag not supported".to_owned());
            }
        }
    }

    Ok(map)
}
