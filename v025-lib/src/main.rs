mod lib;

use lib::pr::test_lib;
use lib::r::R;
fn main() {
    test_lib();
    let mut r = R::new("Rust".to_string(), 1);
    println!("{}", r.get_name());
    r.set_name("Golang".to_string());
    println!("{}", r.get_name());
}
