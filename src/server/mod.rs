use futures::{future, Future, Stream};
use hyper::{
    http::Result,
    service::{make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use std::net::SocketAddr;

// Helpers to parse incomming requests
fn get_method<A>(req: &Request<A>) -> &Method {
    req.method()
}

fn get_path_segments<A>(req: &Request<A>) -> Vec<&str> {
    req.uri().path().trim_matches('/').split('/').collect()
}

// Routes
// POST /topic/{topic} -> sends body to topic
// GET /topic/{topic}?limit=X -> streams messages from topic with optional limit
//
// match (get_method(&req), get_path_segments(&req).as_slice()) {
//(&Method::GET, ["pets"]) => {
//let limit = get_query_parameter(&req, "limit").unwrap_or(25);
//let offset = get_query_parameter(&req, "offset").unwrap_or(0);
//let res = list_pets(limit, offset).unwrap();

//Box::new(future::ok(res))
//}
//(&Method::POST, ["pets"]) => {
//let res = req
//.into_body()
//.concat2()
//.map(|body| serde_json::from_slice::<Pet>(&body).unwrap())
//.map(|pet| create_pets(pet).unwrap());

//Box::new(res)
//}
//// Swagger specification has pet_id as String in parameter, but u64 in schema
//// Using u64 accross the board for consistency
//(&Method::GET, ["pets", pet_id]) if pet_id.parse::<u64>().is_ok() => {
//let typed_pet_id = pet_id.parse::<u64>().unwrap();
//let res = show_pet_by_id(typed_pet_id).unwrap();
//Box::new(future::ok(res))
//}

async fn router(req: Request<Body>) -> Result<Response<Body>> {
    match (get_method(&req), get_path_segments(&req).as_slice()) {
        (&Method::GET, ["topic", topic]) if topic.parse::<String>().is_ok() => {
            let typed_topic = topic.parse::<String>().unwrap();
            let message = format!("Getting messages from topic '{}'", typed_topic);
            Ok(Response::new(Body::from(message)))
        }
        (&Method::POST, ["topic", topic]) if topic.parse::<String>().is_ok() => {
            let typed_topic = topic.parse::<String>().unwrap();
            let message = format!("Sending message to topic '{}'", typed_topic);
            Ok(Response::new(Body::from(message)))
        }
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

async fn shutdown_signal() {
    let shutdown = tokio::signal::ctrl_c().await;
    match shutdown {
        Ok(_) => println!("Gracefully shutting down"),
        Err(e) => eprintln!("Server Error: {}", e),
    };
}

pub async fn start(addr: SocketAddr) {
    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(router)) });
    let server = Server::bind(&addr).serve(service);

    println!("Address: {}", addr);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    if let Err(e) = graceful.await {
        eprintln!("server error: {}", e);
    }
}
