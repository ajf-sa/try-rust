use tide::{
    utils::{After, Before},
    Request, Response,
};

use crate::handler::order_shoes;

#[derive(Debug)]
struct User {
    name: String,
}
pub async fn routes(mut app: tide::Server<()>) -> tide::Server<()> {
    app.with(After(|r: Response| async move {
        let r = match r.status() {
            _ => r,
        };
        println!("mid");
        Ok(r)
    }));
    app.with(Before(|mut req: Request<User>| async move {
        req.set_ext("std::time::Instant::now()");
        req
    }));
    app.at("/").get(|req: Request<()>| async move {
        println!("{:?}", "user");
        Ok(format!("ll"))
    });

    app.at("/api/").nest({
        let mut api = tide::new();
        api.at("/").get(|_| async { Ok("hoowef") });
        api.at("/auth").get(order_shoes);
        api
    });
    app
}
