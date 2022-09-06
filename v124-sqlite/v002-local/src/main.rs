use std::env;

use fake::{
    faker::{
        internet::en::FreeEmail,
        name::raw::{FirstName, LastName},
        phone_number::zh_cn::PhoneNumber,
    },
    locales::AR_SA,
    Fake,
};

fn main() {
    let first_name: String = FirstName(AR_SA).fake();
    let last_name: String = LastName(AR_SA).fake();
    let email: String = FreeEmail().fake();
    let phone: String = PhoneNumber().fake();

    env::set_var("RUST_BACKTRACE", "1");
    let connection = sqlite::open("./db.sqlite3").unwrap();
    match connection.execute(
        "CREATE TABLE contacts (
contact_id INTEGER PRIMARY KEY,
first_name TEXT NOT NULL,
last_name TEXT NOT NULL,
email TEXT NOT NULL,
phone TEXT NOT NULL 
);",
    ) {
        Ok(_) => {
            println!("OK");
        }
        Err(e) => {
            if e.to_string() != String::from("table contacts already exists (code 1)") {
                println!("{}", e)
            }
        }
    };

    connection
        .execute(format!(
            "insert into contacts (first_name,last_name,email,phone) values(\"{}\",\"{}\",\"{}\",\"{}\")",
            first_name, last_name, email,phone,
        ))
        .unwrap();

    connection
        .iterate("select * from contacts", |p| {
            for &(column, value) in p {
                print!(" {} = {}", column, value.unwrap());
            }
            print!("\n");
            true
        })
        .unwrap();
}
