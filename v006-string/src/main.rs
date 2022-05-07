fn main() {
    // literal string
    let name :&str = "ahmed";
    println!("his name is {}", name);
    // string slice or object string
   let uname :String = String::new();
    println!("his length name is {}", uname.len());
    let fname :String = String::from(name);
    println!("his length name is {}", fname.len());

    let mut string = String::new();
    string.push_str("Hello");
    string.push_str(" World");
    println!("{}", string);

    let name3 = "hello there , how are you today ?";
    println!("{}", name3);

    let name4 = name3.replace("hello", "Hello");
    println!("{}", name4);

    let mut company = "Tutorial".to_string();
    company.push('s');
    println!("{}",company);

    let  fulname = " ahmed ahmed ahmed  ";
    println!("{}",fulname.trim());
}
