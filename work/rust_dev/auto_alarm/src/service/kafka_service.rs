use crate::common::*;
use crate::dtos::data_obj::*;
use crate::service::tele_bot_service::*;
use crate::service::es_service::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct KafkaBroker {
    pub brokers: String,
}


impl KafkaBroker {

    /*
        Constructor of Kafka Client
    */
    pub fn new(kafka_host: &str) -> Result<Self, KafkaError> {
        
        let kafka_client = KafkaBroker {
            brokers: kafka_host.to_string()
        };

        Ok(kafka_client)
    }
    
    /*
        Function that creates a consumer object
    */
    fn create_consumer(&self, topic: &str) -> Result<StreamConsumer, KafkaError> {
        
        let consumer_res = ClientConfig::new()
            .set("group.id", "nosql_mon_consumer")
            .set("bootstrap.servers", &self.brokers)
            .set("enable.auto.commit", "true")
            .set_log_level(RDKafkaLogLevel::Info)
            .create();
         
        // StreamConsumer creation result processing
        let consumer: StreamConsumer = consumer_res?;
        consumer.subscribe(&[topic])?;

        Ok(consumer)
    }
    
    /*
        Asynchronous function that consumes messages from a specific topic.
    */
    pub async fn consume_and_send_messages(&self, topic: &str, es_client: EsHelper, shared_res: Arc<SharedResource>) -> Result<(), anyhow::Error> {

        let consumer: StreamConsumer = self.create_consumer(topic)?;

        let mut message_stream = consumer.stream();

        while let Some(message) = message_stream.next().await {
            
            match message {
                
                Ok(m) => {
                    
                    if let Some(payload) = m.payload_view::<str>() {
                        
                        let payload_data = match payload {
                            Ok(payload_data) => payload_data,
                            Err(err) => {
                                error!("{:?}", err);
                                continue;
                            }
                        };
                        
                        let payload_json: Value = match serde_json::from_str(payload_data) {
                            Ok(res) => res,
                            Err(err) => {
                                error!("{:?}", err);
                                continue;
                            }
                        };
                        
                        let alarm_type = match payload_json.get("alarm_type").and_then(Value::as_str).ok_or("alarm_type parsing error") {
                            Ok(alarm_type) => alarm_type,
                            Err(err) => {
                                error!("{:?}",err);
                                continue;
                            }
                        };
                        
                        let telegram_data: tokio::sync::RwLockReadGuard<Vec<Telebot>> = shared_res.tele_data.read().await;

                        match alarm_type {

                            "error_alarm" => {
                                
                                let monitor_metric_obj:AlarmMetricForm<AlarmDetailError> = match serde_json::from_str(payload_data) {
                                    Ok(monitor_metric_obj) => monitor_metric_obj,
                                    Err(err) => {
                                        error!("{:?}", err);
                                        continue;
                                    }
                                };
                                
                                for tele_bot in &*telegram_data {
                                    tele_bot.process_msg(&monitor_metric_obj, &es_client).await?;
                                }

                            },
                            "metric_alarm" => {
                                
                                let monitor_metric_obj:AlarmMetricForm<AlarmDetailInfo> = match serde_json::from_str(payload_data) {
                                    Ok(monitor_metric_obj) => monitor_metric_obj,
                                    Err(err) => {
                                        error!("{:?}", err);
                                        continue;
                                    }
                                };
                                
                                // Send Message to Telegram Bot
                                for tele_bot in &*telegram_data {
                                    tele_bot.process_msg(&monitor_metric_obj, &es_client).await?;
                                }
                            },
                            _ => {
                                error!("Unknown alarm type");
                                continue;
                            }
                        }
                        
                    } else {
                        error!("No payload in message");
                    }
                },
                Err(e) => error!("Error receiving message: {:?}", e),
            }
        }
        
        Ok(())
    }

}
