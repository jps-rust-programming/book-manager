use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{self, File};
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
#[derive(Deserialize, Serialize, Debug)]
struct Book {
    title: String,
    author: String,
    published_year: u32,
}
#[derive(Default)]
struct BookManager {
    books: Vec<Book>,
}

impl BookManager {
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }

    fn list_books(&self) {
        for book in &self.books {
            println!("{:?}", book);
        }
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let json = serde_json::to_string(&self.books)?;
        let mut file = File::create(filename)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
    fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        if Path::new(filename).exists() {
            let mut file = File::open(filename)?;
            let mut json = String::new();
            file.read_to_string(&mut json)?;
            // file.read_to_string(&mut json)?;
            self.books = serde_json::from_str(&json)?;
        }
        Ok(())
    }
}
fn main() {
    let mut manager = BookManager::default();
    manager.load_from_file("books.json").unwrap_or_default();
    loop {
        println!("\nBook Manager");
        println!("1. Add Book");
        println!("2. List Books");
        println!("3. Save Books");
        println!("4. Exit");
        print!("Choose an option: ");
        let stdin = io::stdin();
        let input = stdin.lock().lines().next().unwrap().unwrap();

        match input.trim() {
            "1" => {
                let mut title = String::new();
                let mut author = String::new();
                let mut year = String::new();
                println!("Enter title: ");
                stdin.lock().read_line(&mut title).unwrap();
                println!("Enter author: ");
                stdin.lock().read_line(&mut author).unwrap();
                println!("Enter published year: ");
                stdin.lock().read_line(&mut year).unwrap();
                let book = Book {
                    title: title.trim().to_string(),
                    author: author.trim().to_string(),
                    published_year: year.trim().parse().unwrap(),
                };
                manager.add_book(book);
            }

            "2" => {
                manager.list_books();
            }

            "3" => {
                manager.save_to_file("books.json").unwrap();
                println!("Books saved to books.json");
            }
            "4" => {
                break;
            }

            _ => {
                println!("Invalid option. Please try again!");
            }
        }
    }
}
