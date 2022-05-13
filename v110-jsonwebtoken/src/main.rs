use serde::{Deserialize, Serialize};

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

#[derive(Debug, Serialize, Deserialize)]
struct Clims {
    sub: String,
    company: String,
    exp: usize,
}
fn main() {
    let my_claims = Clims {
        sub: "12923".to_owned(),
        company: "aramco".to_owned(),
        exp: 10000000000,
    };
    let key = b"secret";
    let mut header = Header::new(Algorithm::HS256);
    header.kid = Some("102390293".to_owned());
    let token = match encode(&header, &my_claims, &EncodingKey::from_secret(key)) {
        Ok(t) => t,
        Err(e) => panic!("{}", e),
    };
    println!("\n");
    println!("this is token: {} \n", token);

    let token_data = match decode::<Clims>(
        &token,
        &DecodingKey::from_secret(key),
        &Validation::new(Algorithm::HS256),
    ) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("{}", err), // Example on how to handle a specific error
            _ => panic!("{}", err),
        },
    };

    println!("{:?}", token_data.claims.company);
    println!("{:?}", token_data.header);
}
