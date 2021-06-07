mod router;

pub use crate::router::Router;
use service::{Message, Service};
use affirm::AffirmService;
use log::LogService;
use darwinia_relayer::DarwiniaRelayerService;

#[async_std::main]
async fn main() -> anyhow::Result<()> {
	// router init
	let mut router = Router::new();

	let mut affirm_service = AffirmService::new(0);
	affirm_service.start().await?;
	router.add(Box::new(affirm_service));

	let mut log_service = LogService::new();
	log_service.start().await?;
	router.add(Box::new(log_service));

	let mut darwinia_relayer_service = DarwiniaRelayerService::new("wss://rpc.darwinia.network", 100000);
	darwinia_relayer_service.start().await?;
	router.add(Box::new(darwinia_relayer_service));

	//
	let msg = Message {
		route_key: "darwinia.affirm.affirm",
		content: "125",
	};
	router.send(msg).await;

	let msg2 = Message {
		route_key: "darwinia.affirm.fuck",
		content: "fuck",
	};
	router.send(msg2).await;

	let msg3 = Message {
		route_key: "darwinia.guard.dingding",
		content: "sfsfsdfsdf",
	};
	router.send(msg3).await;
	Ok(())
}
