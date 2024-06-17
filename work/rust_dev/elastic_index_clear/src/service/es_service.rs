use crate::common::*;

use crate::utils_modules::time_utils::*;

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
        Functions that handle queries at the Elasticsearch Cluster LEVEL
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
    

}


impl EsObj {


    /*
        Function that EXECUTES elasticsearch queries
    */
    pub async fn node_search_query(&self, es_query: &Value, index_name: &str) -> Result<Value, anyhow::Error> {

        //info!("{} host executed the query.",self.es_host);
        
        // Response Of ES-Query
        let response = self.es_pool
            .search(SearchParts::Index(&[index_name]))
            .body(es_query)
            .send()
            .await?;
        
        info!("{:?}", response);

        if response.status_code().is_success() { 
            let response_body = response.json::<Value>().await?;
            Ok(response_body)
        } else {
            Err(anyhow!("response status is failed"))
        }
    }
    

    

}
