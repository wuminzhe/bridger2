mod error;
mod darwinia_tracker;

use service::{Service, Message};

#[macro_use]
extern crate log;

pub type Error = error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub struct DarwiniaRelayerService {
}

impl DarwiniaRelayerService {
    pub fn new() -> Self {
        DarwiniaRelayerService {  }
    }
}

#[async_trait::async_trait]
impl Service for DarwiniaRelayerService {
    fn get_binding_keys(&self) -> Vec<&'static str> {
        vec![
            "*.darwinia.*"
        ]
    }

    async fn send(&self, msg: Message) {
        println!("Message log - [ route_key: {}, content: {} ]", msg.route_key, msg.content);
    }
}
