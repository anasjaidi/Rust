use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

fn get_file_path(filename: &str) -> String {
    let current_dir = env::current_exe().unwrap();
    let tmp = current_dir.with_file_name(filename.to_owned());
    tmp.to_str().unwrap().to_owned()
}

fn get_file_content_as_string(path: String) -> Result<String, String> {
    let file = fs::read(path);

    match file {
        Err(err) => Err(err.to_string()),
        Ok(file) => Ok(String::from_utf8_lossy(&file).to_string()),
    }
}

fn get_count_word(content: String, word_to_search: &str) -> i32 {

    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in content.split(' ') {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    *map.get(word_to_search).unwrap_or(&0)
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("args error");
        process::exit(1);
    }
    let filename = args.remove(1);
    let word_to_search = args.remove(1);

    let content = match get_file_content_as_string(get_file_path(&filename)) {
        Ok(content) => content,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    };

    let count = get_count_word(content, &word_to_search);

    println!("word: {} count: {}", word_to_search, count)
}
