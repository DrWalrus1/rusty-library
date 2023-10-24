use crate::book::{Book, BorrowStatus};

mod book;

fn borrow_book(mut book: Book, name: String) -> Book {
    return match book.is_available {
        BorrowStatus::Available => {
            book.is_available = BorrowStatus::Borrowed { borrower: name.clone() };
            println!("{} successfully borrowed: {}!", &name, book.name);
            book
        }
        BorrowStatus::Borrowed { borrower} => {
            println!("Book: {} is already borrowed by {}", &book.name, &borrower);
            book.is_available = BorrowStatus::Available;
            book
        }
    }
}

fn return_book(mut book: Book) -> Book {
    book.is_available = BorrowStatus::Available;
    println!("Book: {} was returned.", &book.name);
    book
}

fn main() {
    let wonderland = Book {
        name: "Alice in Wonderland".to_string(),
        pages: 250,
        is_available: BorrowStatus::Available,
    };
    let mut hunger_games = Book {
        name: "Hunger Games".to_string(),
        pages: 500,
        is_available: BorrowStatus::Borrowed { borrower: "Bob".to_string() },
    };

    borrow_book(wonderland, "Jonathon".to_string());
    hunger_games = borrow_book(hunger_games, "Jonathon".to_string());

    hunger_games = return_book(hunger_games);
    borrow_book(hunger_games, "Jonathon".to_string());
}
