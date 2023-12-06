use crate::book::{Book, BorrowStatus};

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



fn main() {
    let mut books: Vec<Book> = load_library();
    let search_result: Option<usize> = search_library_for_book_by_int(&books, 2);
    match search_result  {
        Some(id) => borrow_book_by_id(&mut books, id, String::from("Jonathan")),
        None => println!("Could not find available book with id")
    }
    let search_result: Option<usize> = search_library_for_book_by_name(&books, String::from("Alice in Wonderland"));
    match search_result  {
        Some(id) => borrow_book_by_id(&mut books, id, String::from("Timmy")),
        None => println!("Could not find available book with id")
    }
    dbg!(&books);
    
    let search_result: Option<usize> = search_library_for_book_by_name(&mut books, String::from("Alice in Wonderland"));
    match search_result  {
        Some(id) => return_book(&mut books, id),
        None => println!("Could not find book with id")
    }
    dbg!(&books);
}
