use tide::Request;

pub async fn order_shoes(mut req: Request<()>) -> tide::Result {
    println!("{}", req.host().unwrap());
    Ok(format!("Hello,! I've put in an order fo shoes").into())
}
