use crate::book::Book;
use crate::status::Status;

#[derive(Debug)]
pub struct BookManager {
    pub books: Vec<Book>,
}

impl BookManager {
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn change_status(&mut self, name: String, new_status: Status) {
        for book in &mut self.books {
            if book.title == name {
                match new_status {
                    Status::Available => book.status = Status::Available,
                    Status::Borrowed => book.status = Status::Borrowed,
                }
            }
        }
    }

    pub fn list_books_with_status(&self, status: Status) {
        let books_with_status: Vec<&Book> = self.books
            .iter()
            .filter(|book| book.status == status)
            .collect();
        dbg!(books_with_status);
    }
}