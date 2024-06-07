use crate::common::*;

use crate::service::plotter_service::*;

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
    
    /*

    */
    pub async fn get_metric_obj_info(&self, cluster_name: &str, host_info: &str, metric_type: &str, size: i32) -> Result<MetricObject, anyhow::Error> {
        
        let cur_time = get_current_utc_time("%Y.%m.%d");
        let index_name = format!("nosql_metric_log-{}",cur_time);
        
        let query = json!({
            "query": {
                "bool": {
                  "filter": [
                    {
                      "term": {
                        "host_info.keyword": host_info
                      }
                    },
                    {
                      "term": {
                        "metric_type.keyword": metric_type
                      }
                    },
                    {
                    "term": {
                        "cluster_name.keyword": cluster_name
                        }
                    }
                  ]
                }
              },
              "sort": [
                {
                  "@timestamp": {
                    "order": "desc" 
                  }
                }
              ],
              "size": size
        });


        let query_res = self.cluster_search_query(query, &index_name).await?;

        if let Some(query_res_vec) = query_res["hits"]["hits"].as_array() {

            let mut metric_data_set_list:Vec<(DateTime<Utc>,f64)> = Vec::new();

            for res_elem in query_res_vec {

                println!("res_elem : {:?}\n=======", res_elem);

                let time_stamp = res_elem["time_stamp"].as_str().unwrap_or_else(|| "none");
                let metric_value = res_elem["metric_type"].as_str().unwrap_or_else(|| "0").parse::<f64>()?;

                let time_stamp_date: DateTime<Utc> = DateTime::parse_from_rfc3339(time_stamp)
                    .map(|dt| dt.with_timezone(&Utc))?;
                
                println!("time_stamp: {:?}", time_stamp);
                println!("metric_value: {:?}", metric_value);
                println!("time_stamp_date: {:?}", time_stamp_date);
                
                metric_data_set_list.push((time_stamp_date, metric_value));
            }
            
            let metric_obj = MetricObject::new(cluster_name.to_string(), metric_type.to_string(), metric_data_set_list);

            return Ok(metric_obj);
        }
        
        Err(anyhow!("All Elasticsearch connections failed"))
    }

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
        
        info!("{:?}", response);

        if response.status_code().is_success() { 
            let response_body = response.json::<Value>().await?;
            Ok(response_body)
        } else {
            Err(anyhow!("response status is failed"))
        }
    }
    

    

}
