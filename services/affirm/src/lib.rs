use xtra::{Address, Actor};
use crate::affirm_actor::{AffirmActor, Affirm};
use service::{Service, Message};

mod affirm_actor;
use xtra::spawn::AsyncStd;

pub struct AffirmService {
	last_relayed: u64,
    actor: Option<Address<AffirmActor>>,
}

impl AffirmService {
    pub fn new(last_relayed: u64) -> Self {
        AffirmService {
			last_relayed,
			actor: None
		}
    }
}

#[async_trait::async_trait]
impl Service for AffirmService {
    fn get_binding_keys(&self) -> Vec<&'static str> {
        vec![
            "darwinia.affirm.*"
        ]
    }

	async fn start(&mut self) -> std::result::Result<(), service::Error> {
        let actor = AffirmActor::new()
            .create(None)
            .spawn(&mut AsyncStd);
		self.actor = Some(actor);
		Ok(())
	}

    async fn send(&self, msg: Message) {
        if msg.route_key.split(".").last() == Some("affirm") {
            let block_number = msg.content.parse::<u64>().unwrap();
			match &self.actor {
				Some(actor) =>  {
					actor.send(Affirm { block_number }).await;
				},
				None => {
					println!("Affirm service has not been started");
				}
			}
        } else {
            println!("Affirm service can not process message: {}, {}", msg.route_key, msg.content);
        }
    }
}
