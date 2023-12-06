use std::ops::ControlFlow;
use crate::menu::MenuOptions;
use crate::library::Library;

mod library;
mod menu;

fn main() {
    let mut library = Library::new();
    let mut username = String::new();
    println!("Please enter username:");
    std::io::stdin().read_line(&mut username).unwrap();
    'program: loop {
        menu::print_menu();
        let mut menu_option_input = String::new();
        std::io::stdin().read_line(&mut menu_option_input).unwrap();
        menu_option_input = String::from(menu_option_input.trim());

        let selected_menu_input = MenuOptions::convert_str_to_menu_option(&menu_option_input);
        if selected_menu_input.is_none() {
            println!("Input is not a valid option");
            continue;
        }
        let selected_menu_input = selected_menu_input.unwrap();

        match selected_menu_input {
            MenuOptions::Borrow => {
                if let ControlFlow::Break(_) = library.borrow_book(&username) {
                    break 'program;
                }
            },
            MenuOptions::Return => println!("Sorry, the library has not implemented this feature yet"),
            MenuOptions::Quit => break,
        }
    }
    println!("Thank you for visiting the library!")
    
}
