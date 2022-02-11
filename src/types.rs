use std::sync::Arc;
use deadqueue::unlimited::Queue;
use futures_util::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use crate::server::WorkOrder;

pub type Tx = UnboundedSender<Message>;
pub type Rx = UnboundedReceiver<Message>;

pub type WsSender = SplitSink<WebSocketStream<TcpStream>, Message>;
pub type WsReceiver = SplitStream<WebSocketStream<TcpStream>>;

pub type WorkQueue = Arc<Queue<WorkOrder>>;