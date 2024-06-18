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

#[derive(Debug, Getters, Clone, new)]
#[getset(get = "pub")]
pub struct MetricInfo {
    pub node_host: String,
    pub total_in_bytes: u64,
    pub free_in_bytes: u64,
    pub available_in_bytes: u64
}

#[derive(Debug, Getters, Clone, new)]
#[getset(get = "pub")]
pub struct IndexInfos {
    pub index_name: String,
    pub store_size: String
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
        Function that calls "CAT" api from Elasticsearch - Elasticsearch Cluster LEVEL
    */
    pub async fn cluster_cat_query(&self, filter_option: &str, limit_cnt: i32) -> Result<VecDeque<IndexInfos>, anyhow::Error> {
        
        for es_obj in self.mon_es_pool.iter() {

            match es_obj.node_cat_query(filter_option, limit_cnt).await {
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
        Function that executes a query that can verify the current disk status of each node. - Elasticsearch Cluster LEVEL
    */
    pub async fn cluster_stats_fs(&self) -> Result<Vec<MetricInfo>, anyhow::Error> {

        for es_obj in self.mon_es_pool.iter() {

            match es_obj.node_stats_fs().await {
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
        Function that calls "DELETE" api from Elasticsearch - Elasticsearch Cluster LEVEL
    */
    pub async fn cluster_delete_query(&self, index_name: &str) -> Result<(), anyhow::Error> {

        for es_obj in self.mon_es_pool.iter() {

            match es_obj.node_delete_query(index_name).await {
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
        Run a query that can confirm the disk status of each node at present.
    */
    async fn node_stats_fs(&self) -> Result<Vec<MetricInfo>, anyhow::Error> {

        // Response Of ES-Query
        let response = self.es_pool
            .nodes()
            .stats(elasticsearch::nodes::NodesStatsParts::Metric(&["fs"]))
            .send()
            .await?;
        
        let mut metric_info_list: Vec<MetricInfo> = Vec::new();
        let query_res: Value = response.json().await?;
        
        if let Some(nodes) = query_res.get("nodes").and_then(Value::as_object) {
            for (_node_id, details) in nodes {
                let node_host = details.get("ip").and_then(Value::as_str).unwrap_or_default().to_string();

                if let Some(fs_data) = details.get("fs").and_then(|fs| fs.get("data")).and_then(Value::as_array) {
                    for disk in fs_data {
                        let total_bytes = disk.get("total_in_bytes").and_then(Value::as_u64).unwrap_or(0);
                        let free_bytes = disk.get("free_in_bytes").and_then(Value::as_u64).unwrap_or(0);
                        let available_bytes = disk.get("available_in_bytes").and_then(Value::as_u64).unwrap_or(0);

                        metric_info_list.push(MetricInfo::new(node_host.clone(), total_bytes, free_bytes, available_bytes));
                    }
                }
            }
        }
        
        Ok(metric_info_list)
    }
    
    
    /*
        Function that calls "CAT" api from Elasticsearch - Elasticsearch Node LEVEL
    */
    async fn node_cat_query(&self, filter_option: &str, limit_cnt: i32) -> Result<VecDeque<IndexInfos>, anyhow::Error> {
        
        // Response Of ES-Query
        let response = self.es_pool
            .cat()
            .indices(elasticsearch::cat::CatIndicesParts::None)
            .format("json")
            .s(&[filter_option])
            .send()
            .await?;
        
        let indices_data: Vec<Value> = response.json().await?;

        let mut index_infos_queue: VecDeque<IndexInfos> = VecDeque::new();
        let mut cnt = 0;

        for index in indices_data {
            
            if cnt == limit_cnt { break; }
            
            let index_name = index.get("index").and_then(Value::as_str).unwrap_or("");

            if ! index_name.starts_with('.') {
                let store_size = index.get("store.size").and_then(Value::as_str).unwrap_or("");
                let index_infos_obj = IndexInfos::new(index_name.to_string(), store_size.to_string());
                index_infos_queue.push_back(index_infos_obj);

                cnt += 1;
            }
        }

        Ok(index_infos_queue)
    }

    
    /*
        Function that calls "DELETE" api from Elasticsearch - Elasticsearch Node LEVEL
    */
    async fn node_delete_query(&self, index_name: &str) -> Result<(), anyhow::Error> {
        
        let response = self.es_pool
            .indices()
            .delete(elasticsearch::indices::IndicesDeleteParts::Index(&[index_name]))
            .send()
            .await?;

            if response.status_code().is_success() {
                info!("Index '{}' has been deleted successfully.", index_name);
            } else {
                // Error response output
                let err_body = response.text().await?;
                error!("Failed to delete index: {}", err_body);
            }
            
        Ok(())
    }

    

}
