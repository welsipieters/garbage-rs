use crate::server::connection::{Connection, ConnectionMap};
use crate::CONNECTION_MANAGER;
use anyhow::Result;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use uuid::Uuid;
use crate::messages::client::ClientMessage;
use crate::server::WorkOrder;
use crate::types::{Rx, Tx, WorkQueue, WsReceiver, WsSender};


pub struct Network;

impl Network {
    pub async fn start_listening(queue: WorkQueue) -> Result<()> {
        let listener = TcpListener::bind("0.0.0.0:8999").await?;
        println!("Server listening on address: 0:0:0:0:8999");

        while let Ok((stream, addr)) = listener.accept().await {
            dbg!(&addr);

            let (tx, rx) = unbounded_channel();

            let connection = Connection::new(addr.clone(), tx.clone());
            let connection_id = connection.id.clone();

            {
                let mut write_lock = CONNECTION_MANAGER.write().await;
                write_lock.add_connection(connection);
            }

            Network::handle_connection(connection_id, stream, rx, Arc::clone(&queue)).await;
        }

        Ok(())
    }

    async fn handle_connection(id: Uuid, stream: TcpStream, rx: Rx, queue: WorkQueue) {
        let websocket = tokio_tungstenite::accept_async(stream)
            .await
            .expect("An error occurred while accepting the content stream");

        let (sender, receiver) = websocket.split();

        tokio::spawn(Network::receive_websocket_message(id, receiver, queue));
        tokio::spawn(Network::receive_stream_message(rx, sender));
    }

    async fn receive_stream_message(mut rx: Rx, mut sender: WsSender) {
        while let Some(message) = rx.recv().await {
            sender.send(message).await;
        }
    }

    async fn receive_websocket_message(id: Uuid, mut receiver: WsReceiver, queue: WorkQueue) {
        while let Some(message) = receiver.next().await {
            let message = message.expect("Error while receiving message from socket.");

            if message.is_close() {
                println!("Received close message.");

                break;
            }

            dbg!(&message);
            queue.push(WorkOrder {
                connection_id: id,
                message: ClientMessage { id: "TEST".to_string(), payload: vec![] }
            });
            dbg!(&queue.len());
        }
    }
}
