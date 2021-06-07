mod router;

pub use crate::router::Router;
use service::Message;
use affirm::AffirmService;
use log::LogService;

#[async_std::main]
async fn main() {
	// router init
	let mut router = Router::new();
	router.add(Box::new(AffirmService::new(0)));
	router.add(Box::new(LogService::new()));

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
}
