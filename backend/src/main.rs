mod endpoints;
mod handlers;
mod routes;

#[tokio::main]
async fn main() {
    let api = routes::init();
    warp::serve(api).run(([127, 0, 0, 1], 9000)).await;
}
