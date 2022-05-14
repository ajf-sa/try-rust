fn main() {
    let hello = String::from("Hello, world!");
    let say = &hello[0..5];
    let s2 = &hello[7..hello.len()];
    println!("{} {}", say, s2);
}
