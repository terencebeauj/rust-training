#[derive(Debug, PartialEq)]
enum Status {
    Available,
    Borrowed,
}

#[derive(Debug)]
struct Book {
    title: String,
    status: Status,
}

#[derive(Debug)]
struct BookManager {
    books: Vec<Book>,
}

impl BookManager {
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn change_status(&mut self, name: String, new_status: Status) {
        for book in &mut self.books {
            if book.title == name {
                match new_status {
                    Status::Available => book.status = Status::Available,
                    Status::Borrowed => book.status = Status::Borrowed,
                }
            }
        }
    }

    fn list_books_with_status(&self, status: Status) {
        let books_with_status: Vec<&Book> = self.books
            .iter()
            .filter(|book| book.status == status)
            .collect();
        dbg!(books_with_status);
    }
}

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
