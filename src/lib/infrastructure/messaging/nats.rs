use std::{fmt::Debug, sync::Arc};

use async_nats::{connect, Client};
use futures::StreamExt;
use serde::de::DeserializeOwned;

use crate::application::ports::{
    messaging_ports::MessagingPort, messaging_subscriber_port::MessagingSubscriberPort,
};

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

#[derive(Clone)]
pub struct NatsMessaging {
    connection: Arc<Client>,
}

impl NatsMessaging {
    pub async fn new(addrs: &str) -> anyhow::Result<NatsMessaging> {
        let client = connect(addrs).await?;

        Ok(NatsMessaging {
            connection: Arc::new(client),
        })
    }

    pub fn get_connection(&self) -> Arc<Client> {
        Arc::clone(&self.connection)
    }
}

impl MessagingPort for NatsMessaging {
    async fn publish_message(&self, topic: String, message: String) -> anyhow::Result<()> {
        let conn = self.get_connection();

        conn.publish(topic, message.into())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))
            .map(|_| ())
    }
}

impl MessagingSubscriberPort for NatsMessaging {
    async fn subscribre<F, T>(&self, topic: &str, handler: F) -> anyhow::Result<()>
    where
        F: Fn(T) -> anyhow::Result<()> + Send + Sync + 'static,
        T: DeserializeOwned + Send + Sync + Debug + 'static,
    {
        let conn = self.get_connection();

        let t = String::from(topic);

        let mut subscriber = conn
            .subscribe(t)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        while let Some(message) = subscriber.next().await {
            let message_str = String::from_utf8_lossy(&message.payload).to_string();

            let parsed_message: T = serde_json::from_str(&message_str)
                .map_err(|e| anyhow::anyhow!("Failed to deserialize message: {:?}", e))?;

            handler(parsed_message)?;
        }

        Ok(())
    }
}
