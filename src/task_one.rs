#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug)]
struct Person {
    name: String,
    age: u16,
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
    is_available: bool,
}

impl Book {
    fn new(title: String, author: String, is_available: bool) -> Self {
        Self { title, author, is_available }
    }
}

#[derive(Debug)]
struct Library {
    books: Vec<Book>,
    borrower_map: HashMap<String, String>
}

impl Library {
    fn new() -> Self {
        Self { books: Vec::new(), borrower_map: HashMap::new() }
    }
    fn add_book(&mut self, title: String, author: String) {
        let book = Book::new(title, author, true);
        self.books.push(book);
    }
    fn print_books(&self) {
        for b in self.books.iter() {
            println!("{:?}", b)
        }
    }

    fn borrow_book(&mut self, title: &String, borrower_name: &String) -> Result<(), String> {
        let _book = match self.books.iter_mut().find(|x| { x.title.eq(title)}) {
            Some(book) => book,
            None => return Err(format!("{} book not found", title))
        };
        return match _book.is_available {
            true => {
                Library::set_book_availability(_book, false);
                self.borrower_map.insert(title.to_string(), borrower_name.to_string());
                Ok(())
            }
            false => {
                Err(format!("Book {} is already checked out.", title))
            }
        }
    }

    fn return_book(&mut self, title: &String) -> Result<(), String>{
        let _book = match self.books.iter_mut().find(|x| x.title == title.to_string()) {
            Some(b) => b,
            None => return Err(format!("Book {} not found", title))
        };
        return match _book.is_available {
            false => {
                self.borrower_map.remove(&_book.title);
                Library::set_book_availability(_book, true);
                Ok(())
            }
            _ => Ok(())
        }
    }

    fn list_available_books(&self) {
        self.books.iter()
            .filter(|x| {x.is_available == true})
            .for_each(|y| {println!("Title : {}, Author: {}", y.title, y.author)})
    }

    fn list_checked_out_books(&self) {
        self.books.iter()
            .filter(|x| {x.is_available == false})
            .for_each(|y| {
                let borrower_name = match self.borrower_map.get(&y.title) {
                    Some(z) => z,
                    None => "No Borrower"
                };
                println!("Title : {}, Borrowed By: {}", y.title, borrower_name)
            })
    }

    fn set_book_availability(book: &mut Book, value: bool) {
        book.is_available = value
    }

}

pub fn run() {
    let person = Person::new(String::from("Dharan"), 25);
    display_person_info(&person);
    let mut library = Library::new();


    let book = library.add_book(String::from("Harry Porter"),
                     String::from("JK Rowling"));
    library.list_available_books();
    library.borrow_book(&String::from("Harry Porter"), &person.name).expect("Err checking out book");
    library.list_checked_out_books();
    library.return_book(&String::from("Harry Porter")).expect("Error returning book");
    library.list_available_books()
}

fn display_person_info(p: &Person) {
    println!("Person(name: {}, age: {})", p.name, p.age)
}