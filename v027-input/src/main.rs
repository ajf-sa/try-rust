fn main() {
    let mut line = String::from("");
    println!("Enter your name :");
    let _ = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello , {}", line);
}
