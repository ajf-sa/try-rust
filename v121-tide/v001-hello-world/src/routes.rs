use crate::handler::order_shoes;

pub async fn routes(mut app: tide::Server<()>) -> tide::Server<()> {
    app.at("/").get(order_shoes);
    app.at("/api").get(order_shoes);
    app
}
