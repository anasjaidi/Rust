use std::{fs::{self, File}, path::{Path, PathBuf}};

pub fn zip(files: &Vec<String>, path: Option<&String>, filename: &String) {

    let f = PathBuf::from("~").join(format!("{}.zip", filename)).to_str().unwrap();
    let file_path = match path {
        None => {
        
        }
        Some(p) => 
    };
    match fs::create_dir_all(file_path) {
        Err(_err) => {}
        _ => {}
    };
    let file = File::create(file_path);

}
