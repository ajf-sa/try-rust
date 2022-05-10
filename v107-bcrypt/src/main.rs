use bcrypt::{hash, verify, DEFAULT_COST};
fn main() {
    let password = "password";
    let hash = hash(password, DEFAULT_COST).unwrap();
    println!("{}", hash);
    let result = verify(password, &hash).unwrap();
    println!("{}", result);
}
