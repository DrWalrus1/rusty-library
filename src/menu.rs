pub enum MenuOptions {
    Borrow = 1,
    Return = 2,
    Quit
}

impl MenuOptions {
    pub fn convert_str_to_menu_option(input: &str) -> Option<MenuOptions> {
        match input {
            "1" => return Some(MenuOptions::Borrow),
            "2" => return Some(MenuOptions::Return),
            "Q" | "q" => return Some(MenuOptions::Quit),
            _ => return None,
        }
    }
}

pub fn print_menu() {
    let menu = 
    r#"Welcome to the Library!
    Please select an option:
    1) Borrow
    2) Return
    q) Quit
    "#;

    println!("{}", menu);
}