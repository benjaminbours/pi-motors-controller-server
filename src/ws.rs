use crate::{direction::Direction, Client, Clients, MotorsControllerArc};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use warp::ws::{Message, WebSocket};

pub async fn client_connection(
    ws: WebSocket,
    id: String,
    clients: Clients,
    mut client: Client,
    motor_controller: MotorsControllerArc,
) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    client.sender = Some(client_sender);
    clients.write().await.insert(id.clone(), client);

    println!("{} connected", id);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error receiving ws message for id: {}): {}", id.clone(), e);
                break;
            }
        };
        client_msg(&id, msg, &clients, &motor_controller).await;
    }

    clients.write().await.remove(&id);
    println!("{}, disconnected", id);
}

async fn client_msg(
    id: &str,
    msg: Message,
    _clients: &Clients,
    motor_controller: &MotorsControllerArc,
) {
    println!("received message from {}: {:?}", id, msg);
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };
    if message == "ping" || message == "ping\n" {
        return;
    }

    let value = message.to_string().parse::<u8>().unwrap();
    println!("value is {}", value.to_string());

    let direction = Direction::from_u8(value);

    match direction {
        Direction::UP => motor_controller.write().await.forward(),
        Direction::DOWN => motor_controller.write().await.backward(),
        Direction::LEFT => motor_controller.write().await.left(),
        Direction::RIGHT => motor_controller.write().await.right(),
        Direction::NONE => motor_controller.write().await.stop(),
    }
}
