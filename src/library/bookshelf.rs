use super::book::Book;

pub struct Bookshelf {
    books: Vec<Book>
}

impl Bookshelf {
    pub fn new() -> Self {
        Self { books: Vec::new() }
    }

    // 本を追加するメソッド
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    // タイトルで本を検索するメソッド
    pub fn search_books(&self, title_query: &str) -> Vec<&Book> {
        todo!("Implement `Bookshelf::search_books`");
    }

    // 本を本棚から取り出すメソッド
    pub fn remove_book(&self, book: &super::book::Book) -> Option<Book> {
        todo!("Implement `Bookshelf::remove_book`");
    }

    // 本棚の本をすべて取り出すメソッド
    pub fn take_all_books(&mut self) -> Vec<Book> {
        todo!("Implement `Bookshelf::take_all_books`");
    }
}
