use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    age: i32,
}

fn open_my_db() -> Result<Connection> {
    let path = "./my_db.db";
    let db = Connection::open(&path)?;
    println!("{}", db.is_autocommit());
    Ok(db)
}

fn main() -> Result<()> {
    let conn = open_my_db()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            age INTEGER NOT NULL
        )",
        params![],
    )
    .unwrap();
    let persion = Person {
        id: 0,
        name: "Steven".to_string(),
        age: 3,
    };
    for _ in 1..100 {
        conn.execute(
            "INSERT INTO person (name, age)
            VALUES (?1, ?2)",
            params![persion.name, persion.age],
        )
        .unwrap();
    }

    let mut stmt = conn.prepare("SELECT id, name, age FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0).unwrap(),
            name: row.get(1).unwrap(),
            age: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}
