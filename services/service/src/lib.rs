#[async_trait::async_trait]
pub trait Service {
    fn get_binding_keys(&self) -> Vec<&'static str>;
    async fn send(&self, msg: Message);
}

#[derive(Clone)]
pub struct Message {
    pub route_key: &'static str,
    pub content: &'static str,
}