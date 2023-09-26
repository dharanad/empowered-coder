use std::fmt::{Display, Formatter, Pointer};

fn print_collection<T: std::fmt::Display>(collection : &[T]) {
    collection.iter().for_each(|x| println!("{x}"))
}
struct Person {
    name: String,
    age: u8
}

impl Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person({}, {})", self.name, self.age)
    }
}

impl Person {
    fn info(&self) -> String {
        format!("Name: {}, age: {}", self.name, self.age)
    }

    pub fn display(&self) {
        println!("Person({})", self.info())
    }

    fn new(name: &str, age: u8) -> Self {
        Self {name: name.to_string(), age }
    }
}

struct Book {
    title: String,
    author: String,
    borrowed_by: Option<String>
}

impl Book {
    pub fn is_available(&self) -> bool {
        return self.borrowed_by.is_none();
    }

    pub fn new(title: String, author: String) -> Self {
        Self {title, author, borrowed_by: None}
    }

    pub fn set_borrower(&mut self, borrower_name: String) {
        self.borrowed_by = Option::from(borrower_name);
    }
    pub fn remove_borrower(&mut self) {
        self.borrowed_by = None
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.borrowed_by {
            Some(by) => {
                write!(f, "Book(Title = {}, Author = {}, BorrowedBy = {})", self.title, self.author, by)
            }
            None => {
                write!(f, "Book({}, {})", self.title, self.author)
            }
        }
    }
}
struct Library {
    name: String,
    books: Vec<Book>
}

impl Library {
    fn list_book(&self, is_available: bool) -> Vec<&Book> {
        self.books
            .iter()
            .filter(|x| x.is_available() == is_available)
            .collect()
    }

    fn find_book(&mut self, title: &str) -> Option<&mut Book> {
        self.books.iter_mut().filter(|x| {x.title == title}).last()
    }

    pub fn list_available_books(&self) {
        print_collection(&self.list_book(true));
    }

    pub fn list_borrowed_books(&self) {
        print_collection(&self.list_book(false));
    }

    pub fn add_book(&mut self, b: Book) -> Result<(), String> {
        self.books.push(b);
        Ok(())
    }

    pub fn borrow_book(&mut self, title: &str, p: &Person) -> Result<&Book, String> {
        match self.find_book(title) {
            Some(b) => {
                b.set_borrower(p.name.clone());
                Ok(b)
            }
            None => {
                Err(format!("err book {} not found", title))
            }
        }
    }

    pub fn return_book(&mut self, title: &str) -> Result<(), String> {
        match self.find_book(title) {
            Some(b) => {
                if !b.is_available() {
                    b.remove_borrower();
                    Ok(())
                } else {
                    Err(format!("err book {} not borrowed", title))
                }
            }
            None => {
                Err(format!("err book {} is not found", title))
            }
        }
    }

    fn new(name: &str) -> Self {
        Library{name: name.to_string(), books: Vec::new() }
    }
}

impl Display for Library {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Library({},count={})", self.name, self.books.len())
    }
}

pub fn run() {
    let mut library = Library::new("Ashram Public School");
    let person = Person::new("Dharan", 25);
    let _ = library.add_book(Book::new("A".to_string(), "Dharan".to_string()));
    let _ = library.add_book(Book::new("B".to_string(), "Satish".to_string()));
    let _ = library.add_book(Book::new("C".to_string(), "Kiran".to_string()));
    let _ = library.add_book(Book::new("D".to_string(), "BT".to_string()));
    library.list_available_books();
    library.list_borrowed_books();
    match library.borrow_book("A", &person) {
        Ok(b) => {
            library.list_borrowed_books();
            let _ = library.return_book("A"); // if used borrowed book two mutable reference exists
            library.list_borrowed_books()
        }
        Err(e) => {
            library.list_available_books()
        }
    }
}