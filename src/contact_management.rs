use std::collections::HashMap;
use std::io::{self, Write};
use std::str::FromStr;

#[derive(Debug)]
pub struct Contact {
    name: String,
    phone: String,
    email: String,
}

pub struct ContactManager {
    contacts: HashMap<String, Contact>,
}

impl ContactManager {
    pub fn new() -> Self {
        ContactManager {
            contacts: HashMap::new(),
        }
    }

    pub fn add_contact(&mut self, name: String, phone: String, email: String) {
        let contact = Contact { name: name.clone(), phone, email };
        self.contacts.insert(name, contact);
    }

    pub fn remove_contact(&mut self, name: &str) {
        self.contacts.remove(name);
    }

    pub fn list_all_contacts(&self) {
        if self.contacts.is_empty() {
            println!("No contacts found");
            return;
        }
        println!("Contacts:");
        for contact in self.contacts.values() {
            println!("Name {}, Phone {}, Email {}", contact.name, contact.phone, contact.email);
        }
    }
}

pub fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn run_contact_management() {
    let mut contact_manager = ContactManager::new();

    loop {
        println!("\nContact Management System");
        println!("1. Add Contact");
        println!("2. Remove Contact");
        println!("3. List Contacts");
        println!("4. Exit");

        let choice = read_input("Choose an option: ");

        match choice.as_str() {
            "1" => {
                let name = read_input("Enter name: ");
                let phone = read_input("Enter phone: ");
                let email = read_input("Enter email: ");
                contact_manager.add_contact(name, phone, email);
                println!("Contact added successfully");
            },
            "2" => {
                let name = read_input("Enter the name of the contact to remove: ");
                contact_manager.remove_contact(&name);
                println!("Contact removed successfully");
            },
            "3" => {
                contact_manager.list_all_contacts();
            },
            "4" => {
                println!("Exiting the application.");
                break;
            },
            _ => {
                println!("Invalid option, please try again.");
            }
        }
    }
}