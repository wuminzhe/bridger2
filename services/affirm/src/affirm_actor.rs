use xtra::{Actor, Message, Handler, Context, Address};

pub struct AffirmActor {
    target: Option<u64>,
}

impl AffirmActor {
    pub fn new() -> Self {
        AffirmActor {
            target: None,
        }
    }
}

impl Actor for AffirmActor {}

pub struct Affirm {
    pub block_number: u64
}

impl Message for Affirm {
    type Result = ();
}

#[async_trait::async_trait]
impl Handler<Affirm> for AffirmActor {
    async fn handle(&mut self, affirm: Affirm, _ctx: &mut Context<Self>) {
        println!("Printing {}.", affirm.block_number);
    }
}