use crate::common::*;
use crate::data_obj::*;
use reqwest::Client;
use reqwest::ClientBuilder;


#[derive(Debug)]
pub struct EsHelper {
    client: Client,
    base_url: String
}


impl EsHelper {
    
    /*
        Constructor of Elasticsearch
    */
    pub async fn new(infos: &NosqlScheduleInfoExtend) -> Result<Self, anyhow::Error> {
        
        let mut host_list = infos.host_port_list().clone();
        let cluster_name = infos.cluster_name();
        let ssl_option = infos.ssl_option();
        let user_id = infos.user_id();
        let user_pw = infos.user_pw();

        let mut flag = true;
        let mut es_url = String::new();

        // Create Client instance.
        let http_client = if *ssl_option {
            ClientBuilder::new()
                .danger_accept_invalid_certs(true)
                .build()?
        } else {
            Client::new()
        };

        /*
            es cluster = {a,b,c,d,e} : nodes
            - One node is randomly selected from the nodes belonging to the cluster.
            - If the connection to the node fails, remove the vector from the node vector and try connecting to another node again.

            <a,c,d,e> -> b : connection fail.
            <c,d,e> -> a : connection try.
        */
        while flag {
            
            if host_list.is_empty() {
                flag = false;
                break;
            }

            // This is to select a random node among the nodes in the cluster.
            let mut rng = StdRng::from_entropy();

            // Random number generation based on vector size
            let index = rng.gen_range(0..host_list.len());
            let selected_node = host_list[index].clone();
            
            if !selected_node.contains(":9200") { continue; }

            let protocol = if *ssl_option { "https" } else { "http" };
            es_url = format!("{}://{}:{}@{}", protocol, user_id, user_pw, selected_node);  
            
            // Cluster Health Check
            let health_check = match Self::cluster_health_check(&http_client, es_url.as_ref(), 5).await {
                Ok(res) => res,
                Err(err) => {
                    error!("A timeout occurred on specific node '{}' in cluster '{}'. Try to connect to another node. : {}", &selected_node, &cluster_name, err);
                    String::from("unknown")
                }
            };
            
            //println!("ping_check : {:?}", ping_check);
            host_list.remove(index);  

            if health_check == "unknown" { continue; }
            else if health_check == "green" || health_check == "yellow" { 
                info!("{} is connected", cluster_name); 
                flag = false
            } else {
                // health_check == "yellow" or "red"
                // This is a situation where an ES cluster health problem has occurred and an alarm must be sent.
                return Err(anyhow!(format!("[{}] health is {}", cluster_name, health_check)));
            }
            
        } // while
        
        if flag {
            return Err(anyhow!(format!("Connection to that cluster is not possible. : [{}]", cluster_name)));
        }
          
        Ok(
            EsHelper {
                client: http_client,
                base_url: es_url
            }
        )

    }
    
    /*
        Function to check Es cluster health
    */
    async fn cluster_health_check(client: &Client, es_url: &str, timeout_sec: u64) -> Result<String, anyhow::Error> {
            
        let resp = client
            .get(format!("{}/_cluster/health?pretty", es_url))
            .timeout(Duration::from_secs(timeout_sec))
            .send()
            .await?
            .text()
            .await?;

        let health_status: Value = serde_json::from_str(&resp)?;
        let status = health_status["status"].as_str().unwrap_or("red").to_string();
        
        Ok(status)
    }
    
    

    /* 
        Function that directly sends a query to ES and receives the RESPONSE result.
    */
    // async fn send_request_query(&self, method: Method, url: &str, query: &Value, timeout_sec: u64) -> Result<Value, reqwest::Error> {
        
    //     let request_builder = self.client.request(method, url).json(query).timeout(Duration::from_secs(timeout_sec));
    //     let response_rst = request_builder.send().await?.json::<Value>().await?;

    //     Ok(response_rst)
    // }
    

    /* 
        Function that directly sends a query to ES and receives the RESPONSE result.
        Used only when a request WITHOUT direct query content is NEEDED.
    */
    async fn send_request_not_query(&self, method: Method, url: &str, timeout_sec: u64) -> Result<Value, reqwest::Error> {
        
        let request_builder = self.client.request(method, url).timeout(Duration::from_secs(timeout_sec));
        let response_rst = request_builder.send().await?.json::<Value>().await?;

        Ok(response_rst)
    }
    

    // /* 
    //     Function that directly sends a query to ES and receives the RESPONSE result(String).
    // */
    async fn send_request_not_query_text(&self, method: Method, url: &str, timeout_sec: u64) -> Result<String, reqwest::Error> {
        
        let request_builder = self.client.request(method, url).timeout(Duration::from_secs(timeout_sec));
        let response_rst = request_builder.send().await?.text().await?;
        
        Ok(response_rst)
    }
    
    
    // /* 
    //     Function to SEARCH data in ElasticSearch
    // */
    // pub async fn es_search(&self, index: &str, query: Value, timeout_sec: u64) -> Result<Value, reqwest::Error> {
        
    //     let url = format!("{}/{}/_search", self.base_url, index);
    //     self.send_request_query( Method::GET, &url, &query, timeout_sec).await
    // }

    /*
        Function to CAT data in Elasticsearch
    */
    pub async fn es_cat(&self, end_point: &str, timeout_sec: u64) -> Result<String, reqwest::Error> {
        let url = format!("{}/_cat/{}", self.base_url, end_point);
        println!("{:?}", url);
        self.send_request_not_query_text( Method::GET, &url, timeout_sec).await
    }
    
    /*
        Function to DELETE date in Elasticsearch
    */
    pub async fn es_delete(&self, index: &str, timeout_sec: u64) -> Result<Value, reqwest::Error> {
        let url = format!("{}/{}",self.base_url, index);
        self.send_request_not_query( Method::DELETE, &url, timeout_sec).await
    }
        
}