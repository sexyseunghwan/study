use crate::common::*;


/*
    
*/
static KAFKA_GLOBAL_CLIENT: Lazy<Result<Arc<FutureProducer>, anyhow::Error>> = Lazy::new(|| {
    init_kafka_global_conn()
});


/*
    
*/
pub fn init_kafka_global_conn() -> Result<Arc<FutureProducer> , anyhow::Error> {

    let kafka_host = env::var("KAFKA_HOST").expect("'KAFKA_HOST' must be set");

    let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", kafka_host)
            .create()?;

    Ok(Arc::new(producer))
}


/*
    
*/
pub async fn logger_kafka_global(topic: &str, message: &str) -> Result<(), anyhow::Error>  {
        
    //let kafka_producer = &self.produce_broker;
    
    let kafka_producer = match KAFKA_GLOBAL_CLIENT.as_ref() {
        Ok(kafka_producer) => kafka_producer,
        Err(e) => {
            return Err(anyhow!(format!("{:?}", e)))
        }
    }.clone();

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
