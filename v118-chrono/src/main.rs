use chrono::prelude::*;
fn main() {
    let utc : DateTime<Utc> = Utc::now();
    let local :DateTime<Local> = Local::now();
    let dt = Utc.ymd(2018, 11, 14).and_hms(8, 9, 10);
    let dt_local = Local.ymd(2018, 11, 14).and_hms(8, 9, 10);
    println!("{}", utc);
    println!("{}", local);
    println!("{}", dt);
    println!("{}", dt_local);
    println!("{}", age_from_birthday("1988-03-28 00:00:00"));
}

fn age_from_birthday(birthday: &str) -> i64 {
    let now = Utc::now();
    let bday = fun_name(birthday);
    let age = now.signed_duration_since(bday).num_days() / 365;
    age
}

fn fun_name(birthday: &str) -> DateTime<Utc> {
    let bday = Utc.datetime_from_str(birthday, "%Y-%m-%d %H:%M:%S");
    let n = match  bday {
        Ok(n) => n,
        Err(e) => panic!(" {}", e),
    };
    println!("{}", n);
    n
}