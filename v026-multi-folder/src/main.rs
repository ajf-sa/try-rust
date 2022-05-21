use crate::{model::second::am_second, model2::fourth::am_fourth};

mod model;
mod model2;

fn main() {
    println!("{}", model::first::am_first());
    println!("{}", am_second());

    println!("{}", model2::third::am_third());
    println!("{}", am_fourth());
}
