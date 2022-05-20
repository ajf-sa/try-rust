fn main() {
    // calling a function
    hello();
    let hi = say_hello();
    println!("I will say {}", hi);
    let n = add(4, 5);
    println!("sum is {}", n);
    let mut no: i32 = 5;
    // to_zero(&mut no);
    back_fn(&mut no)(&mut no);
    println!("The value of no is:{}", no);
}
// defining function
fn hello() {
    println!("this is first function on rust");
}

fn say_hello() -> String {
    "HI".to_string()
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn to_zero(p: &mut i32) {
    *p = 0;
}

fn back_fn(s: &mut i32) -> fn(&mut i32) {
    to_zero
}
