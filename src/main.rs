extern crate core;

use crate::server::connection::ConnectionMap;
use crate::server::Server;
use anyhow::Result;
use tokio::sync::RwLock;
use crate::messages::client::ClientMessageManager;

#[macro_use]
extern crate lazy_static;

mod constants;
mod level;
mod prefabs;
mod server;
mod t_helpers;
mod types;
mod messages;


// I should do something about this?
lazy_static! {
    pub static ref CONNECTION_MANAGER: RwLock<ConnectionMap> =
        { RwLock::new(ConnectionMap::new()) };

    pub static ref MESSAGE_MANAGER: RwLock<ClientMessageManager> = {
        RwLock::new(ClientMessageManager::new())
    };
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut server = Server::new();
    server.boot();
    server.start().await;

    Ok(())
}
