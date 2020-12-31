mod server;

#[tokio::main]
async fn main() {
    let addr = ([0, 0, 0, 0], 8080).into();

    server::start(addr).await
}
