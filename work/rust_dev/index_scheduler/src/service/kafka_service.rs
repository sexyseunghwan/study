use crate::common::*;

use crate::dtos::data_obj::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KafkaBroker {
    pub brokers: String,
    
}

#[derive(Clone)]
pub struct ProduceBroker {
    produce_broker: FutureProducer
}


/*
    Function to send a warning alarm to a specific topic in Kafka
*/
pub async fn send_message_to_kafka_alarm<T: AlarmDetail>(mon_metric_form: &AlarmMetricForm<T>, kafka_client: &ProduceBroker, topic_name: &str) -> Result<(), anyhow::Error> {

    let mon_metric_form = serde_json::to_string(&mon_metric_form)?;
    let mon_metric_form_str = mon_metric_form.as_str();
    
    match kafka_client.produce_message(topic_name, mon_metric_form_str).await {
        Ok(_) => (),
        Err(err) => {
            return Err(anyhow!(err.to_string()));
        }
    }
    
    Ok(())
}


impl ProduceBroker {
    
    /*
        Constructor of Kafka Producer
    */
    pub fn new(kafka_host: &str) -> Result<Self, anyhow::Error> {
        
        let kafka_client = KafkaBroker {
            brokers: kafka_host.to_string()
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
            Ok(_) => { Ok(()) },
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
