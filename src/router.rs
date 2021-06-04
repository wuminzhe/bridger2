use service::{Service, Message};

pub struct Router {
    services: Vec<Box<dyn Service>>,
}

impl Router {
    pub fn new() -> Router {
        Self {
            services: vec![]
        }
    }

    pub fn add(&mut self, service: Box<dyn Service>) {
        self.services.push(service);
    }

    pub async fn send(&self, msg: Message) {
        for service in &self.services {
            let binding_keys = service.get_binding_keys();
            if self.matched(binding_keys, msg.route_key) {
                service.send(msg.clone()).await
            }
        }
    }

    fn matched(&self, binding_keys: Vec<&str>, route_key: &str) -> bool {
        let route_segs: Vec<&str> = route_key.split(".").collect();
        for binding_key in binding_keys {
            let segs: Vec<&str> = binding_key.split(".").collect();
            if (segs[0] == "*" || segs[0] == route_segs[0]) &&
                (segs[1] == "*" || segs[1] == route_segs[1]) &&
                (segs[2] == "*" ||  segs[2] == route_segs[2] )
            {
                return true;
            }
        }

        false
    }
}
