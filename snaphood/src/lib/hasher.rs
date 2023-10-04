use std::collections::HashMap;
use std::fs::*;
use std::time::SystemTime;

use crate::data::Flag;

pub fn get_hasher(map: &HashMap<Flag, Vec<String>>) -> String {

    let mut hash = String::new();

    for (_, v) in map {
        for f in v {
            let md = metadata(f).unwrap();
            let modified_time = md.modified().unwrap();

            let modified_time_in_ms = modified_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis().to_string();
            hash += &modified_time_in_ms;
        }
    }

    hash
}