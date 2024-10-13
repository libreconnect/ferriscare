use std::sync::Arc;

use async_nats::{connect, Client};

pub struct Nats {
    client: Arc<Client>,
}

impl Nats {
    pub async fn new() -> anyhow::Result<Nats> {
        let client = connect("127.0.0.1:4222").await?;

        Ok(Nats {
            client: Arc::new(client),
        })
    }

    pub fn get_client(&self) -> Arc<Client> {
        Arc::clone(&self.client)
    }
}
