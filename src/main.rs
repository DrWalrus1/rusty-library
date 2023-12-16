use std::io::{self, Write};
use std::ops::ControlFlow;
use crate::menu::MenuOptions;
use crate::library::Library;

mod library;
mod menu;

fn main() {
    let mut library = Library::new();
    let mut username = String::new();
    print!("Please enter username: ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut username).unwrap();
    'program: loop {
        menu::print_menu();
        let mut menu_option_input = String::new();
        print!("Selection: ");
        io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut menu_option_input).unwrap();
        menu_option_input = String::from(menu_option_input.trim());

        let selected_menu_input = MenuOptions::convert_str_to_menu_option(&menu_option_input);
        if selected_menu_input.is_none() {
            println!("\"{}\" is not a valid option", menu_option_input);
            continue;
        }
        let selected_menu_input = selected_menu_input.unwrap();

        match selected_menu_input {
            MenuOptions::Borrow => {
                if let ControlFlow::Break(_) = library.borrow_book(&username) {
                    break 'program;
                }
            },
            MenuOptions::Return => {
                let books_borrowed_by_user = library.search_for_books_borrowed_by_borrower(&username);
                if books_borrowed_by_user.is_empty() {
                    println!("You have no books to return!");
                    continue;
                }
                library.print_books_borrowed_by_borrower(&username);
                let mut return_book_input = String::new();
                print!("Please enter the ID of the book you would like to return: ");
                io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut return_book_input).unwrap();
                if let Ok(book_id) = return_book_input.trim().parse::<u32>() {
                    let book_index = library.search_library_for_book_by_int(book_id).unwrap();
                    library.return_book_by_book_id(book_index);
                } else {
                    println!("Invalid input!");
                    continue;
                }
            },
            MenuOptions::Quit => break,
        }
    }
    println!("Thank you for visiting the library!")
    
}
