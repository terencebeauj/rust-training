mod book_manager;
mod status;
mod book;

use crate::book_manager::BookManager;
use crate::status::Status;
use crate::book::Book;

fn main() {
    let status1 = Status::Available;
    let status2 = Status::Available;
    let status3 = Status::Available;

    let book1 = Book {
        title: String::from("Harry Potter 1"),
        status: status1,
    };

    let book2 = Book {
        title: String::from("Calculus"),
        status: status2,
    };

    let book3 = Book {
        title: String::from("Rust Programming"),
        status: status3,
    };

    let mut book_manager = BookManager { books: vec![] };

    book_manager.add_book(book1);
    book_manager.add_book(book2);
    book_manager.add_book(book3);

    dbg!(&book_manager);

    book_manager.change_status(String::from("Calculus"), Status::Borrowed);

    dbg!(&book_manager);

    book_manager.list_books_with_status(Status::Available);
}
