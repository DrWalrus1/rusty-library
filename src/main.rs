use BorrowStatus::Borrowed;
use crate::book::{Book, BorrowStatus};

mod book;

fn borrow_book(mut book: Book) -> Option<Book> {
    match book.is_available {
        BorrowStatus::Available => {
            book.is_available = Borrowed;
            return Some(book);
        }
        Borrowed => {
            None
        }
    }
}

fn main() {
    let x = Book {
        name: "Alice in Wonderland".to_string(),
        pages: 250,
        is_available: BorrowStatus::Available
    };

    match borrow_book(x) {
        None => println!("Book is already borrowed"),
        Some(book) => println!("Successfully borrowed: {}!", book.name)
    }
}
