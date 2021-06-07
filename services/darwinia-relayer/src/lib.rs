mod error;
mod darwinia_tracker;

use service::{Service, Message};
use darwinia::Darwinia;
use darwinia_tracker::DarwiniaBlockTracker;

#[macro_use]
extern crate log;

pub type Error = error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub struct DarwiniaRelayerService {
	url: &'static str,
	scan_from: u32,
}

impl DarwiniaRelayerService {
	pub fn new(url: &'static str, scan_from: u32) -> Self {
		DarwiniaRelayerService {
			url,
			scan_from
		}
	}
}

#[async_trait::async_trait]
impl Service for DarwiniaRelayerService {
	fn get_binding_keys(&self) -> Vec<&'static str> {
		vec![
			"*.darwinia.*"
		]
	}

	async fn start(&mut self) -> std::result::Result<(), service::Error>{
		match Darwinia::new(self.url).await {
			Ok(darwinia) => {
				let mut tracker = DarwiniaBlockTracker::new(darwinia, self.scan_from);

				loop {
					match tracker.next_block().await {
						Ok(header) => {
							println!("{:?}", header);
						},
						Err(_err) => {
							return self.start().await;
						}
					}
				}
			},
			Err(_err) => {
				return Err(service::Error::Stopped("未能连接上 Darwinia 节点, 请检查网络"));
			}
		}
	}

	async fn send(&self, msg: Message) {
		println!("Message log - [ route_key: {}, content: {} ]", msg.route_key, msg.content);
	}
}

