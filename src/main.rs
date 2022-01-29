use tide::Request;
use tide::prelude::*;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    // add hello world method to root
    app.at("/").get(|_| async move {
        Ok("Hello, world!")
    });
    app.at("/orders/shoes").post(order_shoes);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}