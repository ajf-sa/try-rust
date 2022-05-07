fn main() {
 let t:(i32, i32) = (1, 2);
    println!("{}", t.0);
    println!("{}", t.1);

    let (x, _) = t;
    println!("{}", x);
    print(t);
}

fn print(t:(i32, i32)) {
    println!("{}", t.0);
    println!("{}", t.1);
}
