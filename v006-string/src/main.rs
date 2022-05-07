fn main() {
    // literal string
    let name :&str = "ahmed";
    println!("his name is {}", name);
    // string slice or object string
   let uname :String = String::new();
    println!("his length name is {}", uname.len());
    let fname :String = String::from(name);
    println!("his length name is {}", fname.len());
}
