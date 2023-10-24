use BorrowStatus::Borrowed;
use crate::book::{Book, BorrowStatus};

mod book;

fn borrow_book(mut book: Book, name: String) -> Book {
    return match book.is_available {
        BorrowStatus::Available => {
            book.is_available = Borrowed;
            book.borrower_name = Some(name);
            println!("Successfully borrowed: {}!", book.name);
            book
        }
        Borrowed => {
            println!("Book is already borrowed by ");
            book
        }
    }
}

fn return_book(mut book: Book) -> Book {
    book.is_available = BorrowStatus::Available;
    book.borrower_name = None;
    book
}

fn main() {
    let mut x = Book {
        name: "Alice in Wonderland".to_string(),
        pages: 250,
        is_available: BorrowStatus::Available,
        borrower_name: None
    };

    x = borrow_book(x, "Jonathon".to_string());

}
