trait Speak {
    fn say_hello(&self) -> String;
    fn say_bye(&self) -> String {
        "Bye".to_string()
    }
}
struct Person1 {
    name: String,
}
impl Speak for Person1 {
    fn say_hello(&self) -> String {
        String::from("Hello, my name is ") + &self.name
    }

    fn say_bye(&self) -> String {
        format!("{} says bye", &self.name)
    }
}
struct Person2 {}
impl Speak for Person2 {
    fn say_hello(&self) -> String {
        String::from("Hello")
    }
}

struct Person3 {}
impl Speak for Person3 {
    fn say_hello(&self) -> String {
        String::from("Hello World!")
    }
}

fn main() {
    loop {
        let p1 = Person1 {
            name: String::from("John"),
        };
        give_greeting(&p1);
        give_bye(&p1);
        let p2 = Person2 {};
        give_greeting(&p2);
        give_bye(&p2);
        let p3 = Person3 {};
        give_greeting(&p3);
        give_bye(&p3);
    }
}

fn give_greeting<T>(p: &T)
where
    T: Speak,
{
    println!("{}", p.say_hello());
}

fn give_bye<T>(p: &T)
where
    T: Speak,
{
    println!("{}", p.say_bye());
}
