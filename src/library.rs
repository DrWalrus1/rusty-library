use std::ops::ControlFlow;
use crate::library::book::{Book, BorrowStatus};
pub mod book;

pub struct Library {
    pub books: Vec<Book>
}

impl Library {
    pub fn new() -> Self {
        let books = vec![
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
        ];

        Self { books }
    }

    fn print_available_book_titles(&self) {
        for book in &self.books {
            match book.is_available {
                BorrowStatus::Available => println!("{}) - {}", book.id, book.name),
                _ => ()
            }
        }
    }

    pub fn borrow_book(&mut self, username: &String) -> ControlFlow<()> {
        self.print_available_book_titles();
        let mut borrow_book_input = String::new();
        std::io::stdin().read_line(&mut borrow_book_input).unwrap();
        if let Ok(book_id) = borrow_book_input.trim().parse::<u32>() {
            let book_index = self.search_library_for_book_by_int(book_id).unwrap();
            self.borrow_book_by_id(book_index, username)
        } else {
            return ControlFlow::Break(());
        }
        ControlFlow::Continue(())
    }

    fn borrow_book_by_id(&mut self, book_id: usize, borrower_name: &String) {
        let book_to_borrow: &mut Book = self.books.get_mut(book_id).unwrap();
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
    
    fn search_library_for_book_by_int(&self, book_id: u32) -> Option<usize> {
        for i in 0..self.books.len() {
            if self.books[i].id == book_id {
                return Some(i);
            }
        }
        None
    }
}