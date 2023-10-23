mod book;

fn main() {
    let x = book::Book {
        name: String::from("Alice in Wonderland"),
        pages: 250
    };
    println!("Hello, world!");
}
