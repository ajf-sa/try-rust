fn main() {
    // let variable_name: data_type = value;
    // let variable_name = value;
    let fees = 25_000;
    let tax = 0.2;
    println!("fees is {} and tax is {}", fees, tax);

    /* 
    fees = 35_000;
    ^^^^^^^^^^^^^ cannot assign twice to immutable variable
    */
    let mut fees2 = 25_000;
    println!("fees2 is {}", fees2);
    fees2 = 35_000;
    println!("fees2 is {}", fees2);

    let age : i32 = 30;
    println!("age is {}", age);
    let name : String = "John".to_string();
    println!("name is {}", name);
}
