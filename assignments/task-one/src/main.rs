#[allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u16
}

impl Person {
    fn new(name: String, age: u16) -> Self {
        Self { name, age }
    }
}

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    is_available: bool
}

impl Book {
    fn new(title: String, author: String, is_available: bool) -> Self {
        Self { title, author, is_available }
    }

}

#[derive(Debug)]
struct Library {
    books: Vec<Book>
}

impl Library {
    fn new() -> Self {
        Self { books: Vec::new() }
    }
    fn add_book(&mut self, book: Book) {
        self.books.push(book)
    }
    fn print_books(&mut self) {
        for i in 0..self.books.len() {
            println!("{:?}", self.books.get(i))
        }
    }
}

fn main() {
    println!("Hello, world!");
    let person = Person::new(String::from("Dharan"), 25);
    let mut library = Library::new();
    let harry_porter = Book::new(
        String::from("Harry Porter"),
        String::from("JK Rowling"),
        true);
    library.add_book(harry_porter);
    library.print_books()
}
