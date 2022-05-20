mod model;
mod model2;

use model::first::am_first;
use model::second::am_second;

use model2::fourth::am_fourth;
use model2::third::am_third;
fn main() {
    println!("{}", am_first());
    println!("{}", am_second());

    println!("{}", am_third());
    println!("{}", am_fourth());
}
