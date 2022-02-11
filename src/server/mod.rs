extern crate futures;

pub mod connection;
mod network;

use futures::{Future, Sink, Stream};

use crate::MESSAGE_MANAGER;
use crate::level::Level;
use crate::server::connection::{Connection, ConnectionMap};
use crate::server::network::Network;
use anyhow::Result;
use futures_util::future::join_all;
use futures_util::stream::SplitStream;
use futures_util::StreamExt;
use std::net::SocketAddr;
use std::process::id;
use std::sync::Arc;
use std::time::Duration;
use deadqueue::unlimited::Queue;
use timer::Timer;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Notify;
use tokio_tungstenite::WebSocketStream;
use uuid::Uuid;
use crate::messages::client::ClientMessage;
use crate::types::WorkQueue;

const RUNNER_AMOUNT: usize = 10;


pub struct Server {
    timer: Timer,
    levels: Vec<Level>,
    work_queue: WorkQueue,
    can_process_work: Arc<Notify>
}

impl Server {
    pub fn new() -> Server {
        Server {
            timer: Timer::new(),
            levels: vec![],
            work_queue: Arc::new(Queue::new()),
            can_process_work: Arc::new(Notify::new())
        }
    }

    pub fn boot(&self) {
        for i in 0..RUNNER_AMOUNT {
            self.spawn_runner(i, Arc::clone(&self.can_process_work));
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        // Start the network
        // Ugliest network implementation ever.
        tokio::task::spawn(Network::start_listening(Arc::clone(&self.work_queue)));

        self.can_process_work.notify_waiters();

        loop {
            tokio::time::sleep(Duration::from_millis(16)).await;
            self.tick().await;
        }
    }

    async fn tick(&self) {
        // todo: i really wanna use a `Notify` for this.
        // Spawn level "runners" independently. String them together by queue.
        // Have all of them on an infinite loop awaiting a `Notify`.
        let futures = self.levels.iter().map(|x| x.tick()).collect::<Vec<_>>();

        // How many levels can we do like this before it goes wrong lol.
        join_all(futures).await;
    }

    fn spawn_runner(&self, id: usize, starter: Arc<Notify>) {
        println!("Spawning Order Runner {}", &id);
        let queue = Arc::clone(&self.work_queue);

        tokio::spawn(async move {
            starter.notified().await;

            let id_inner = id;
            println!("Order runner {} started processing", &id_inner);
            let queue_inner = queue;

            while let order = queue_inner.pop().await {
                println!("Order Runner '{}' received order: \r\n {:?}", &id_inner, &order);

                MESSAGE_MANAGER.read().await.handle(order.message);
            }
        });
    }
}

#[derive(Debug)]
pub struct WorkOrder {
    connection_id: Uuid,
    message: ClientMessage
}