use anyhow::Result;
use hashbrown::HashMap;
use std::net::SocketAddr;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;
use crate::types::Tx;

pub struct Connection {
    pub id: Uuid,
    pub addr: SocketAddr,
    pub sink: Tx,
}

impl Connection {
    pub fn new(addr: SocketAddr, sink: UnboundedSender<Message>) -> Connection {
        Connection {
            addr,
            sink,
            id: Uuid::new_v4(),
        }
    }
}

pub struct ConnectionMap {
    map: HashMap<Uuid, Connection>,
}

impl ConnectionMap {
    pub fn new() -> ConnectionMap {
        ConnectionMap {
            map: HashMap::new(),
        }
    }

    pub fn add_connection(&mut self, connection: Connection) -> Result<()> {
        if self.map.contains_key(&connection.id) {
            panic!("ID \"{}\" connected twice.", &connection.id);
        }

        self.map.insert(connection.id.clone(), connection);
        Ok(())
    }

    pub fn get_connection(&self, id: &Uuid) -> Option<&Connection> {
        self.map.get(id)
    }
}
