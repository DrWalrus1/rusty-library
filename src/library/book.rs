#[derive(Debug)]
pub enum BorrowStatus {
    Available,
    Borrowed { borrower: String }
}

#[derive(Debug)]
pub struct Book {
    pub id: u32,
    pub name: String,
    pub pages: u32,
    pub is_available: BorrowStatus
}