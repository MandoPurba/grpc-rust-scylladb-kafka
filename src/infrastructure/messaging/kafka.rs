use anyhow::Ok;
use rdkafka::{
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};

use crate::domain::{event_publisher::EventPublisher, models::Note};

pub struct KafkaEventPublisher {
    producer: FutureProducer,
    topic: String,
}

impl KafkaEventPublisher {
    pub fn new(brokers: &str, topic: &str) -> anyhow::Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()?;

        Ok(Self {
            producer,
            topic: topic.to_string(),
        })
    }
}

#[async_trait::async_trait]
impl EventPublisher for KafkaEventPublisher {
    async fn publish_note_created(&self, note: &Note) -> anyhow::Result<()> {
        let payload = serde_json::to_string(&note)?;
        let key = note.id.to_string();

        let _ = self
            .producer
            .send(
                FutureRecord::to(&self.topic).payload(&payload).key(&key),
                std::time::Duration::from_secs(5),
            )
            .await;

        Ok(())
    }
}
