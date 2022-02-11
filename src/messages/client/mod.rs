use async_trait::async_trait;
use anyhow::anyhow;
use hashbrown::HashMap;

#[derive(Debug, Clone)]
pub struct ClientMessage {
    pub id: String,
    pub(crate) payload: Vec<u8>
}

#[async_trait]
pub trait ClientMessageHandler: Send + Sync {
    fn get_id(&self) -> String;
    async fn handle(&self, message: ClientMessage);
}

pub struct ClientMessageManager {
    handlers: HashMap<String, Box<dyn ClientMessageHandler>>
}

impl ClientMessageManager {
    pub fn new() -> ClientMessageManager {
        ClientMessageManager {
            handlers: HashMap::new()
        }
    }

    pub fn register_handler<Handler: ClientMessageHandler + Send + Sync + 'static>(&mut self, handler: Handler) -> &ClientMessageManager {
        if self.handlers.contains_key(&handler.get_id()) {
            // obv this shouldnt panic lol
            panic!("Handler registered twice.");
        }

        self.handlers.insert(handler.get_id(), Box::new(handler));

        self
    }

    pub async fn handle(&self, message: ClientMessage) {
        if !self.handlers.contains_key(&message.id) {
            // obv this shouldnt panic either
            panic!("unknown message id {}", &message.id);
            // panic!("Client {} sent message with unknown identifier: {}", &connection.read().addr, message.get_identifier());
        }

        let handler = self.handlers.get(&message.id).expect("This should never go wrong. (famous last words)");
        handler.handle(message).await;
    }
}