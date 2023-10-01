use std::env;
use snaphood::data::*;
fn main() {
    // Retrieve the provided file paths passed as arguments
    let args: Vec<String> = env::args().skip(1).collect();

    let allowed_flags = get_allowed_flags();

    let map = snaphood::parser::parse_input(&args, &allowed_flags);

    let map = match map {
        Err(err) => {
            err.print();
            std::process::exit(1);
        },
        Ok(map) => map
    };
    let _map = dbg!(map);
}