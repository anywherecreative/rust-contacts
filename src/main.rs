use std::collections::HashMap;
#[macro_use] extern crate text_io;

fn main() {
    let mut contacts = HashMap::new();
    let mut choice = "".to_string();
    while choice != "q" {
        choice = show_menu().to_lowercase();
        if choice == "a"  {
            contacts.insert(0,add_contact());
            println!("Contact added!\n")
        }
        else if choice == "l"  {
            show_contacts(&contacts);
        }
        else if choice == "q" {
            print_title("Good Bye!".to_string());
        }
        else {
            println!("Invalid selection!");
        }
    }
}

struct Contact {
    first_name: String,
    last_name: String,
    email: String,
    phone: String
}

fn add_contact() -> Contact {
    let mut contact = Contact{
        first_name: "".to_string(),
        last_name: "".to_string(),
        email: "".to_string(),
        phone: "".to_string(),
    };

    print_title("Add A New Contact:".to_string());
    print!("First Name: ");
    let first_name: String = read!("{}\n");
    contact.first_name = first_name;

    print!("Last Name: ");
    let last_name: String = read!("{}\n");
    contact.last_name = last_name;

    print!("Email: ");
    let email: String = read!("{}\n");
    contact.email = email;

    print!("Phone: ");
    let phone: String = read!("{}\n");
    contact.phone = phone;

    contact
}

fn show_contacts(contacts: &HashMap<u32, Contact>) {
    for (_key, contact) in contacts {
        print!("First: {}\nLast: {}\nEmail: {}\nPhone: {}",contact.first_name, contact.last_name, contact.email, contact.phone);
    }
}

fn print_title(title: String) {
    let underline = std::iter::repeat("-").take(title.len()).collect::<String>();
    println!("{}\n{}",title,underline);
}

fn show_menu() -> String {
    print_title("Main Menu".to_string());
    println!("(A)dd Contact");
    println!("(L)ist Contacts");
    println!("(Q)uit");

    read!("{}\n")
}
