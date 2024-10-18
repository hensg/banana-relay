use events::*;
use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use uuid::Uuid;
use warp::ws::{Message, WebSocket};
use warp::Filter;

mod events;

#[derive(Debug, Clone)]
struct Client {
    uuid: Uuid,
    sender: mpsc::UnboundedSender<Result<Message, warp::Error>>,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let clients = Arc::new(Mutex::new(HashMap::new()));

    let ws_route = warp::path("relay")
        .and(warp::ws())
        .and(with_clients(clients.clone()))
        .map(|ws: warp::ws::Ws, clients| {
            ws.on_upgrade(move |socket| client_connected(socket, clients))
        });

    warp::serve(ws_route).run(([127, 0, 0, 1], 3030)).await;
}

fn with_clients(
    clients: Arc<Mutex<HashMap<Uuid, Client>>>,
) -> impl Filter<Extract = (Arc<Mutex<HashMap<Uuid, Client>>>,), Error = std::convert::Infallible> + Clone
{
    warp::any().map(move || clients.clone())
}
//
// UNWRAP everywhere, no mercy...
//
async fn client_connected(ws: WebSocket, clients: Arc<Mutex<HashMap<Uuid, Client>>>) {
    let (tx, mut rx) = ws.split();

    let (client_tx, client_rx) = mpsc::unbounded_channel();
    let client_uuid = Uuid::new_v4();

    let client = Client {
        uuid: client_uuid,
        sender: client_tx,
    };
    clients.lock().unwrap().insert(client_uuid, client.clone());

    println!("Client connected: {}", client_uuid);

    tokio::spawn(async move {
        while let Some(result) = rx.next().await {
            match result {
                Ok(msg) => {
                    if let Ok(text) = msg.to_str() {
                        match serde_json::from_str::<ClientMessage>(text) {
                            Ok(ClientMessage::Notice(_)) => println!("Got a notice"),
                            Ok(ClientMessage::Event(event)) => {
                                println!("Received event: {:?}", event);
                                let clients = clients.lock().unwrap();
                                for (uuid, client) in clients.iter() {
                                    // FIXME: avoid sending the event to itself
                                    let msg = serde_json::to_string(&ServerMessage::Event(
                                        client_uuid.to_string(),
                                        event.clone(),
                                    ))
                                    .unwrap();
                                    println!("Pushing event to client {uuid:?}");
                                    let _ = client.sender.send(Ok(Message::text(msg)));
                                }
                            }
                            Ok(ClientMessage::Req(_, _)) => println!("Got a request"),
                            Ok(ClientMessage::Close(_)) => println!("Got a close"),
                            Err(e) => println!("Invalid client message: {:?}", e),
                        }
                    }
                }
                Err(e) => {
                    println!("WebSocket error: {}", e);
                    break;
                }
            }
        }

        clients.lock().unwrap().remove(&client_uuid);
        println!("Client disconnected: {}", client_uuid);
    });

    tokio::spawn(async move {
        let mut tx = tx;
        let mut client_rx = client_rx;
        while let Some(result) = client_rx.recv().await {
            let _ = tx.send(result.unwrap()).await;
        }
    });
}
