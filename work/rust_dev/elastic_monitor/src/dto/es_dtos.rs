use crate::common::*;


#[derive(Debug, FromRow, Serialize, Deserialize, Getters, Setters, Default, new)]
#[getset(get = "pub", set = "pub")]
pub struct ESMetricInfo {
    pub cluster_name: String,
    pub kibana_url: String,
    pub user_id: Option<String>,
    pub user_pw_enc: Option<Vec<u8>>,
    pub system_version: String,
    pub ssl_option: bool,
    pub shard_limit: f64,
    pub disk_limit: f64,
    pub cpu_limit: f64,
    pub jvm_limit: f64
}

#[derive(Debug, Clone, Serialize, Deserialize, Getters, Setters, Default, new)]
#[getset(get = "pub", set = "pub")]
pub struct ESMetricInfoExtend {
    pub cluster_name: String,
    pub kibana_url: String,
    pub user_id: String,
    pub user_pw: String,
    pub system_version: String,
    pub ssl_option: bool,
    pub shard_limit: f64,
    pub disk_limit: f64,
    pub cpu_limit: f64,
    pub jvm_limit: f64,
    pub host_port_list: Vec<String>
}