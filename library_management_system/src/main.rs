mod author;
mod book;
mod book_genre;
mod library;

use crate::author::Author;
use crate::book::Book;
use crate::book_genre::BookGenre;
use crate::library::Library;

fn main() {
    let author1: Author = Author {
        firstname: String::from("Alan"),
        lastname: String::from("Thomas"),
    };

    let author2: Author = Author {
        firstname: String::from("Lisa"),
        lastname: String::from("Simpson"),
    };

    let author3: Author = Author {
        firstname: String::from("Jeff"),
        lastname: String::from("Tyler"),
    };

    let author4: Author = Author {
        firstname: String::from("Marine"),
        lastname: String::from("Lauwrence"),
    };

    let action: BookGenre = BookGenre::Action;
    let horror: BookGenre = BookGenre::Horror;
    let romance: BookGenre = BookGenre::Romance;
    let thriller: BookGenre = BookGenre::Thriller;
    let fantasy: BookGenre = BookGenre::Fantasy;
    let scifi: BookGenre = BookGenre::SciFi;

    let book1: Book = Book {
        title: String::from("Remember"),
        author: author1.clone(),
        genre: horror.clone(),
    };

    let book2: Book = Book {
        title: String::from("In the Future"),
        author: author3,
        genre: scifi,
    };

    let book3: Book = Book {
        title: String::from("Your Destiny"),
        author: author1.clone(),
        genre: fantasy,
    };

    let book4: Book = Book {
        title: String::from("Together"),
        genre: romance,
        author: author2,
    };

    let book5: Book = Book {
        title: String::from("Help!"),
        author: author4,
        genre: horror.clone(),
    };

    let mut library: Library = Library {
        books: vec![book1, book2, book3, book4, book5.clone()],
    };

    library.add_book(book5.clone());

    dbg!(&library);

    library.remove_book("Help!");

    dbg!(&library);

    let author_books: Vec<&Book> = library.find_books_by_author(author1.clone());

    dbg!(&author_books);

    let horror_books: Vec<&Book> = library.list_books_by_genre(horror.clone());

    dbg!(&horror_books);
}
