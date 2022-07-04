use tide::prelude::*;
use tide::Request;

use crate::handler::order_shoes;
use crate::routes::routes;

mod handler;
mod routes;
#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    let app = routes(app).await;
    app.listen("0.0.0.0:3333").await;
    Ok(())
}
