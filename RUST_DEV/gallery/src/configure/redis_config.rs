use redis::cluster_async::ClusterConnection;

use crate::common::*;


#[derive(Getters, Setters, Clone, new)]
#[getset(get = "pub")]

pub struct RedisHelper {
    pub cluster_client: ClusterClient
}

impl std::fmt::Debug for RedisHelper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("RedisHelper")
            .field("redis_client", &"ClusterClient { ... }")
            .finish()
    }
}

/*

*/
pub fn init_redis_conn() -> Result<ClusterClient, anyhow::Error> {

    let redis_auth = env::var("REDIS_AUTH").expect("'REDIS_AUTH' must be set");

    let redis_hosts: Vec<String> = 
        env::var("REDIS_INFO").expect("'REDIS_INFO' must be set")
            .split(',')
            .map(|addr| format!("redis://:{}@{}", redis_auth, addr))
            .collect();

    let cluster_client = ClusterClient::new(redis_hosts)?; 

    Ok(cluster_client)
}


impl RedisHelper { 

    
    pub async fn get_redis_conn(&self) -> Result<ClusterConnection, anyhow::Error> {
        
        let client = self.cluster_client.clone();
        let conn:ClusterConnection = client.get_async_connection().await?;
        
        Ok(conn)
    }

}