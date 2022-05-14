use fake::faker::internet::raw::FreeEmail;
use fake::faker::phone_number::raw::PhoneNumber;
use fake::locales::EN;
use fake::{Dummy, Fake, Faker};

#[derive(Dummy)]
struct Authorize {
    id: i32,
    email: String,
    phone: String,
    password: String,
}

fn main() {
    let mut f: Authorize = Faker.fake();
    f.email = FreeEmail(EN).fake();
    f.phone = PhoneNumber(EN).fake();
    println!("{} {} {} {}", f.id, f.email, f.phone, f.password);
    let mut space = "  ";
    space = space.len().to_string().as_str();
}
