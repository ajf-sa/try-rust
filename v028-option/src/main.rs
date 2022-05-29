fn main() {
    let v = vec![1, 2, 3];
    match v.get(20) {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}
