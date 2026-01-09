use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use anyhow::Result;
use tracing::{info, error};

pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub fn new(brokers: &str, topic: &str) -> Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .set("acks", "all")
            .create()?;

        Ok(Self {
            producer,
            topic: topic.to_string(),
        })
    }

    pub async fn send_receipt(&self, receipt_json: &str) -> Result<()> {
        let topic = self.topic.clone();
        let key = uuid::Uuid::new_v4().to_string();
        let record = FutureRecord::to(&topic)
            .key(&key)
            .payload(receipt_json);

        match self.producer.send(record, std::time::Duration::from_secs(0)).await {
            Ok(_) => {
                info!("Receipt sent to Kafka topic: {}", self.topic);
                Ok(())
            }
            Err((e, _)) => {
                error!("Failed to send receipt to Kafka: {}", e);
                Err(anyhow::anyhow!("Kafka send error: {}", e))
            }
        }
    }
}

