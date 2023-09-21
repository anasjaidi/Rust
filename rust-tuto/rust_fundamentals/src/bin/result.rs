struct Error {
    name: String,
    code: u8,
}

enum MainMenu {
    Platnium(u8),
    Premium,
    Standard,
}

fn get_menu(str: &str) -> Result<MainMenu, Error> {
    match str {
        "platnium" => Ok(MainMenu::Platnium(0)),
        "premium" => Ok(MainMenu::Premium),
        "standard" => Ok(MainMenu::Standard),
        _ => Err(Error {
            name: "invalid_choice".to_owned(),
            code: 112,
        }),
    }
}

fn main() {
    let choice = "platnium";
    let res = get_menu(choice);
    match res {
        Err(err) => {
            print!("error occured ");
            println!(
                "error name: {} for debug search on this code {}",
                err.name, err.code
            );
        }
        Ok(menu_choice) => match menu_choice {
            MainMenu::Platnium(grade) => {
                println!("you choised platium choice with grade of {}", grade)
            }
            MainMenu::Premium => {
                println!("you choised premium")
            }
            MainMenu::Standard => {
                println!("you choised standard")
            }
        },
    }
}
