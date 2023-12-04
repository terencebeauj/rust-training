use crate::author::Author;
use crate::book_genre::BookGenre;

#[derive(Clone, Debug)]
pub struct Book {
    pub title: String,
    pub author: Author,
    pub genre: BookGenre,
}
