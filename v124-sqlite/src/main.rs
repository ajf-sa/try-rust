fn main() {
    let connection = sqlite::open(":memory:").unwrap();
    connection
        .execute(
            "
        CREATE TABLE users (name TEXT, age INTEGER);
        INSERT INTO users VALUES ('Alice', 42);
        INSERT INTO users VALUES ('Bob', 69);
        ",
        )
        .unwrap();
    connection
        .iterate("SELECT * FROM users WHERE age > 50", |p| {
            for &(c, v) in p.iter() {
                println!("{} = {}", c, v.unwrap());
            }
            true
        })
        .unwrap();
}
