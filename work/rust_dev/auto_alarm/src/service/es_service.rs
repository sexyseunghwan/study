use crate::common::*;

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct EsHelper {
    mon_es_pool: Vec<EsObj>
}

#[derive(Debug, Getters, Clone, new)]
#[getset(get = "pub")]
pub struct EsObj {
    es_host: String,
    es_pool: Elasticsearch
}


impl EsHelper {

    /* 
        Constructor
    */
    pub fn new(es_url_vec: Vec<String>, es_id: &str, es_pw: &str) -> Result<Self, anyhow::Error> {
        
        let mut mon_es_clients: Vec<EsObj> = Vec::new();
    
        for url in es_url_vec {
    
            let parse_url = format!("http://{}:{}@{}", es_id, es_pw, url);
    
            let es_url = Url::parse(&parse_url)?;
            let conn_pool = SingleNodeConnectionPool::new(es_url);
            let transport = TransportBuilder::new(conn_pool)
                .timeout(Duration::new(5,0))
                .build()?;
            
            mon_es_clients.push(EsObj::new(url, Elasticsearch::new(transport)));
        }
        
        Ok(EsHelper{mon_es_pool: mon_es_clients})
    }

    /*

    */
    pub async fn cluster_search_query(&self, es_query: Value, index_name: &str) -> Result<Value, anyhow::Error> {

        for es_obj in self.mon_es_pool.iter() {

            match es_obj.node_search_query(&es_query, index_name).await {
                Ok(resp) => return Ok(resp),
                Err(err) => {
                    error!("{:?}", err);      
                    continue;
                }
            }   
        }

        Err(anyhow!("All Elasticsearch connections failed"))
        
    }


    /*

    */
    // pub async fn conn_es_from_pool(&self) -> Result<EsObj, anyhow::Error> {

    //     info!("come");

    //     let mut rng = StdRng::from_entropy();
        
    //     let mut es_clients = self.mon_es_pool.clone();
    //     es_clients.shuffle(&mut rng);


    //     for es_obj in es_clients.into_iter() {
            
    //         info!("{:?}", es_obj);
            
    //         // let response: elasticsearch::http::response::Response = es_obj.es_pool.ping().send().await?;
            
    //         // if response.status_code().is_success() {
    //         //     info!("Connected to Elasticsearch!");
    //         //     return Ok(es_obj);

    //         // } else {
    //         //     error!("Failed to connect to Elasticsearch. Status code: {}", response.status_code());
    //         //     continue;
    //         // }
    //     }
        
    //     // All nodes in the cluster failed to connect
    //     Err(anyhow!("All Elasticsearch connections failed"))
    // }


}


impl EsObj {


    /*
        Function that EXECUTES elasticsearch queries
    */
    pub async fn node_search_query(&self, es_query: &Value, index_name: &str) -> Result<Value, anyhow::Error> {

        info!("{} host executed the query.",self.es_host);
        
        // Response Of ES-Query
        let response = self.es_pool
            .search(SearchParts::Index(&[index_name]))
            .body(es_query)
            .send()
            .await?;
        
        if response.status_code().is_success() { 
            let response_body = response.json::<Value>().await?;
            Ok(response_body)
        } else {
            Err(anyhow!("response status is failed"))
        }
    }
    

    

}
