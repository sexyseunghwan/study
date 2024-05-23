use crate::common::*;

use redis::cluster_async::ClusterConnection;

/*
    Redis connection-related global variables and initialization logic.
    - The initialization proceeds as soon as the corresponding global variable is approached for the first time.
*/
static REDIS_CLIENT: Lazy<Result<Arc<ClusterClient>, anyhow::Error>> = Lazy::new(|| {
    init_redis_conn()
});


/*
    Function that initializes the Redis connection global variable.
*/
pub fn init_redis_conn() -> Result<Arc<ClusterClient>, anyhow::Error> {
    
    let redis_auth = env::var("REDIS_AUTH").expect("'REDIS_AUTH' must be set");
    
    let redis_hosts: Vec<String> = 
        env::var("REDIS_INFO").expect("'REDIS_INFO' must be set")
            .split(',')
            .map(|addr| format!("redis://:{}@{}", redis_auth, addr))
            .collect();

    let cluster_client = Arc::new(ClusterClient::new(redis_hosts)?);
    Ok(cluster_client)
}


/*
    Function that allows the Redis connection global variable to be thread-safe.
*/
pub async fn get_global_redis_conn() -> Result<ClusterConnection, anyhow::Error> {

    let client = match REDIS_CLIENT.as_ref() {
        Ok(client) => client,
        Err(e) => {
            return Err(anyhow!(format!("{:?}", e)))
        }
    }.clone();
    
    let conn = client.get_async_connection().await?;

    Ok(conn)
}