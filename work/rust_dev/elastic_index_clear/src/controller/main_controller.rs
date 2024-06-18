
use crate::common::*;

use crate::service::es_service::*;
use crate::service::index_clear_service::*;

/*
    ======================================================
    ============= Monitoring Main Controller =============
    ======================================================
*/
pub async fn main_controller() {

    info!("Program Start");

    // Select compilation environment
    dotenv().ok();
    let es_host: Vec<String> = env::var("ES_DB_URL").expect("'ES_DB_URL' must be set").split(",").map(|s| s.to_string()).collect();
    let es_id = env::var("ES_ID").expect("'ES_ID' must be set");
    let es_pw = env::var("ES_PW").expect("'ES_PW' must be set");


    // Elasticsearch connection
    let es_client: EsHelper = match EsHelper::new(es_host, &es_id, &es_pw) {
        Ok(mysql_client) => mysql_client,
        Err(err) => {
            error!("Failed to create mysql client: {:?}", err);
            panic!("Failed to create mysql client: {:?}", err);
        }
    }; 
    

    loop {
        
        match clear_service(&es_client).await {
            Ok(_) => (),
            Err(e) => { error!("{:?}", e) }
        }

        thread::sleep(Duration::from_secs(60));
    }

    
}