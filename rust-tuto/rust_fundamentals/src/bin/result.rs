#[allow(unused)]
struct Error {
    name: String,
    code: u8,
    stack: Vec<String>,
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
            stack: vec![str.to_owned()],
        }),
    }
}

fn main() {
    let choice = "platniumu";
    let res = get_menu(choice);
    match res {
        Err(err) => match err {
            Error {
                name, code: 112, ..
            } if name == "invalid_choice".to_owned() => {
                println!("choice slected is not valid");
            }
            other => {
                print!("error occured ");
                println!(
                    "error name: {} for debug search on this code {}",
                    other.name, other.code
                );
            }
        },
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
    let d  = bar(Err("anas".to_owned()));
    println!("{:?}", d)
}

fn bar(opt: Result<i32, String>) -> Result<(), String> {
    let a = opt?;
    println!("{:?}",a);
    Ok(())
}
