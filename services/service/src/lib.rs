use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
	#[error("The service has stopped. {0}")]
	Stopped(&'static str),
}

#[async_trait::async_trait]
pub trait Service {
    fn get_binding_keys(&self) -> Vec<&'static str>;
	async fn start(&mut self) -> std::result::Result<(), Error>;
    async fn send(&self, msg: Message);
}

#[derive(Clone)]
pub struct Message {
    pub route_key: &'static str,
    pub content: &'static str,
}
