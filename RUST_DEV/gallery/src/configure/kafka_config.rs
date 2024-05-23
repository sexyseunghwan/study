use crate::common::*;


#[derive(Clone, new)]
pub struct ProduceBroker {
    produce_broker: FutureProducer
}

/*
    
*/
pub fn init_kafka_conn() -> Result<ProduceBroker, anyhow::Error> {

    let kafka_host = env::var("KAFKA_HOST").expect("'KAFKA_HOST' must be set");

    let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", kafka_host)
            .create()?;

    let kafka_conn = ProduceBroker::new(producer);

    Ok(kafka_conn)
}


impl ProduceBroker {
    
    /* 
        Kafka Function that produces messages on a specific topic
    */
    pub async fn logger_kafka(&self, topic: &str, message: &str) -> Result<(), anyhow::Error>  {
        
        let kafka_producer = &self.produce_broker;
        
        let record = FutureRecord::to(topic)
            .payload(message)
            .key("");  // You can set a key for the message if needed
        
        match kafka_producer.send(record, Duration::from_secs(5)).await {
            Ok(_) => {
                Ok(())
            },
            Err((e, _)) => Err(anyhow!(e.to_string())),
        }
    }

}