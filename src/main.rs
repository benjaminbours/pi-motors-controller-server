use crate::motors::MotorsController;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{mpsc, RwLock};
use warp::{ws::Message, Filter, Rejection};
mod handler;
mod motors;
mod ws;
mod direction;

#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: usize,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<RwLock<HashMap<String, Client>>>;
type MotorsControllerArc = Arc<RwLock<MotorsController>>;

#[tokio::main]
async fn main() {
    let motor_controller: MotorsControllerArc =
        Arc::new(RwLock::new(motors::MotorsController::new().unwrap()));
    let clients: Clients = Arc::new(RwLock::new(HashMap::new()));

    let health_route = warp::path!("health").and_then(handler::health_handler);

    let register = warp::path("register");
    let register_routes = register
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(handler::register_handler)
        .or(register
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_clients(clients.clone()))
            .and_then(handler::unregister_handler));

    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(warp::path::param())
        .and(with_clients(clients.clone()))
        .and(with_motor_controller(motor_controller.clone()))
        .and_then(handler::ws_handler);

    let routes = health_route.or(register_routes).or(ws_route).with(
        warp::cors()
            .allow_any_origin()
            .allow_headers(vec![
                "User-Agent",
                "Sec-Fetch-Mode",
                "Referer",
                "Origin",
                "Access-Control-Request-Method",
                "Access-Control-Request-Headers",
                "Content-Type",
            ])
            .allow_methods(vec!["POST", "GET"]),
    );

    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

fn with_motor_controller(
    motor_controller: MotorsControllerArc,
) -> impl Filter<Extract = (MotorsControllerArc,), Error = Infallible> + Clone {
    warp::any().map(move || motor_controller.clone())
}
