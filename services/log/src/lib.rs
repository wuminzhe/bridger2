use xtra::{Address, Actor};
use xtra::spawn::Smol;
use service::{Service, Message};

pub struct LogService {
}

impl LogService {
    pub fn new() -> Self {
        LogService {  }
    }
}

#[async_trait::async_trait]
impl Service for LogService {
    fn get_binding_keys(&self) -> Vec<&'static str> {
        vec![
            "*.*.*"
        ]
    }

    async fn send(&self, msg: Message) {
        println!("Message log - [ route_key: {}, content: {} ]", msg.route_key, msg.content);
    }
}