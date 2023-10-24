#[derive(Debug)]
pub enum BorrowStatus {
    Available,
    Borrowed
}

#[derive(Debug)]
pub struct Book {
    pub name: String,
    pub pages: u32,
    pub is_available: BorrowStatus,
    pub borrower_name: Option<String>
}

impl Book {
    pub fn print_book_details(&self) {
        println!("{:?}", self);
        // println!("Book name: {}\nPages: {}", self.name, self.pages);
    }
}