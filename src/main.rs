use crate::book::{Book, BorrowStatus};
use crate::menu::{MenuOptions};

mod menu;
mod book;

fn borrow_book_by_id(books: &mut Vec<Book>, book_id: usize, borrower_name: String) {
    let book_to_borrow: &mut Book = books.get_mut(book_id).unwrap();
    match &book_to_borrow.is_available {
        BorrowStatus::Available => {
            book_to_borrow.is_available = BorrowStatus::Borrowed { borrower: borrower_name.clone() };
            println!("{} successfully borrowed: {}!", &borrower_name, book_to_borrow.name);
        }
        BorrowStatus::Borrowed { borrower} => {
            println!("Book: {} is already borrowed by {}", &book_to_borrow.name, &borrower);
        }
    }
}

fn search_library_for_book_by_int(books: &Vec<Book>, book_id: u32) -> Option<usize> {
    for i in 0..books.len() {
        if books[i].id == book_id {
            return Some(i);
        }
    }
    None
}

fn search_library_for_book_by_name(books: &Vec<Book>, book_name: String) -> Option<usize> {
    for i in 0..books.len() {
        if books[i].name.as_str() == book_name.as_str() {
            return Some(i);
        }
    }
    None
}

fn return_book(books: &mut Vec<Book>, book_id: usize) {
    books.get_mut(book_id).unwrap().is_available = BorrowStatus::Available;
}

fn load_library() -> Vec<Book> {
    vec![
        Book {
            id: 1,
            name: "Alice in Wonderland".to_string(),
            pages: 250,
            is_available: BorrowStatus::Available,
        },
        Book {
            id: 2,
            name: "Hunger Games".to_string(),
            pages: 500,
            is_available: BorrowStatus::Borrowed { borrower: "Bob".to_string() },
        }
    ]
}

fn print_available_book_titles(books: &Vec<Book>) {
    for book in books {
        match book.is_available {
            BorrowStatus::Available => println!("{}) - {}", book.id, book.name),
            _ => ()
        }
    }
}





fn main() {
    let books: Vec<Book> = load_library();
    loop {
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
                print_available_book_titles(&books);
            },
            MenuOptions::Return => println!("Sorry, the library has not implemented this feature yet"),
            MenuOptions::Quit => break,
        }
    }
    println!("Thank you for visiting the library!")
    
}
