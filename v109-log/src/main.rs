use log::{info, warn};
use simple_logger::SimpleLogger;
fn main() {
    SimpleLogger::new().init().unwrap();
    warn!("Hello, world!");
    info!("Hello, world!");
}
