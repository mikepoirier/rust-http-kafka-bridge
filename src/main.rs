use hyper::{
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};

async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let method = req.method();
    let path = req.uri().path();

    Ok(Response::new(Body::from(format!(
        "Method: {}, Path: {}",
        method, path
    ))))
}

async fn shutdown_signal() {
    let shutdown = tokio::signal::ctrl_c().await;
    match shutdown {
        Ok(_) => println!("Gracefully shutting down"),
        Err(e) => eprintln!("Server Error: {}", e),
    };
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();

    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo)) });

    let server = Server::bind(&addr).serve(service);

    println!("Address: {}", addr);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
