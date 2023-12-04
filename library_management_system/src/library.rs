use crate::author::Author;
use crate::book::Book;
use crate::book_genre::BookGenre;

#[derive(Debug)]
pub struct Library {
    pub books: Vec<Book>,
}

impl Library {
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    pub fn remove_book(&mut self, title: &str) {
        let mut index: Option<usize> = None;

        for (i, book) in self.books.iter().enumerate() {
            if book.title == String::from(title) {
                index = Some(i);
                break;
            }
        }

        match index {
            Some(i) => {
                self.books.remove(i);
            }
            None => println!("This book is not in the library"),
        }
    }

    pub fn find_books_by_author(&self, author: Author) -> Vec<&Book> {
        let mut results: Vec<&Book> = vec![];

        for book in &self.books {
            if book.author.firstname == author.firstname && book.author.lastname == author.lastname
            {
                results.push(book);
            }
        }

        results
    }

    pub fn list_books_by_genre(&self, genre: BookGenre) -> Vec<&Book> {
        let mut results: Vec<&Book> = vec![];

        for book in &self.books {
            if book.genre == genre {
                results.push(book);
            }
        }

        results
    }
}
