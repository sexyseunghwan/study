use crate::common::*;

#[derive(Debug, Getters, new)]
#[getset(get = "pub")]
pub struct EsHelper {
    mon_es_pool: Vec<EsObj>
}

#[derive(Debug, Getters, new)]
#[getset(get = "pub")]
pub struct EsObj {
    es_host: String,
    es_pool: Elasticsearch
}

/*
    Function to initialize Elasticsearch connection
*/
pub fn init_es_conn_list(es_url_vec: Vec<String>, es_id: &str, es_pw: &str) -> Result<EsHelper, anyhow::Error> {

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

    Ok(EsHelper::new(mon_es_clients))
    
}


impl EsObj {


    /*

    */
    // async fn es_pool_connection(&self) -> Result<&Elasticsearch, anyhow::Error> {



    //     Ok()
    // }

    /*
        Function that EXECUTES elasticsearch queries
    */
    pub async fn es_search_query(&self, es_query: Value, index_name: &str) -> Result<Value, anyhow::Error> {

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
