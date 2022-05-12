use std::fs::File;
fn main() {
 let f = File::open("imag.jpg");
 match f {
     Ok(f) => {
         println!("{:?}",f);
     },
     Err(e) => {
         println!("{}",e);
     }
 }
}


