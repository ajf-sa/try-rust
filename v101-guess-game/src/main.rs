use rand::random;
use std::cmp::Ordering;
use std::io::stdin;
fn main() {
    println!("Guess the number!");

    let secret_number = (random::<u32>() % 100) + 1;
    loop {
        let mut number = String::from("");
        println!("Please input your guess.");
        let _ = stdin().read_line(&mut number).unwrap();
        let input_number: Result<u32, _> = number.trim().parse();
        let num = match input_number {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number!");
                continue;
            }
        };

        println!("You guessed: {}", num);
        match cmp(num, secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return;
            }
        }
    }
}
fn cmp(a: u32, b: u32) -> Ordering {
    if a < b {
        Ordering::Less
    } else if a > b {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
