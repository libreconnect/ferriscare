use std::{fmt::Debug, future::Future};

use serde::de::DeserializeOwned;

pub trait MessagingSubscriberPort: Clone + Send + Sync + 'static {
    fn subscribre<F, T>(
        &self,
        topic: &str,
        handler: F,
    ) -> impl Future<Output = anyhow::Result<()>> + Send
    where
        F: Fn(T) -> anyhow::Result<()> + Send + Sync + 'static,
        T: DeserializeOwned + Send + Sync + Debug + 'static;
}
