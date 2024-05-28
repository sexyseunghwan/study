use crate::common::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KafkaBroker {
    pub brokers: String,
    
}

#[derive(Clone)]
pub struct ProduceBroker {
    produce_broker: FutureProducer
}


impl ProduceBroker {
    
    /*
        Constructor of Kafka Producer
    */
    pub fn new(c_version: &str) -> Result<Self, anyhow::Error> {
        
        let kafka_file_path = if c_version == "test" || c_version == "local" {
            "./data_file/kafka_info/kafka_info_dev.json"
        } else if c_version == "debug" {
            "./data_file/kafka_info/kafka_info_debug.json"
        } else {
            "./data_file/kafka_info/kafka_info_prod.json"
        };
        
        let parse_kafka_data = fs::read_to_string(kafka_file_path).expect("An error occurred while opening the kafka information file."); 
        let kafka_info_json: KafkaBroker = serde_json::from_str(&parse_kafka_data).expect("An error occurred while parsing the Kafka information file json information.");
        
        let kafka_client = KafkaBroker {
            brokers: kafka_info_json.brokers
        };
        
        let kafka_producer = kafka_client.create_producer()?;
        
        let produce_client = ProduceBroker {
            produce_broker: kafka_producer
        };
        
        Ok(produce_client)
    }
    
    
    /* 
        Kafka Function that produces messages on a specific topic
    */
    pub async fn produce_message(&self, topic: &str, message: &str) -> Result<(), anyhow::Error>  {

        let kafka_producer = &self.produce_broker;
        
        let record = FutureRecord::to(topic)
            .payload(message)
            .key("");  // You can set a key for the message if needed
        
        match kafka_producer.send(record, Duration::from_secs(5)).await {
            Ok(_) => {
                //info!("Successfully produced message on topic '{}'", topic);
                Ok(())
            },
            Err((e, _)) => Err(anyhow!(e.to_string())),
        }
    }

}



impl KafkaBroker { 
    
    /*
        Function that creates a Producer object
    */
    pub fn create_producer(&self) -> Result<FutureProducer, anyhow::Error> {
        
        let producer:FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &self.brokers)
            .create()?;
        
        Ok(producer)
    }
}