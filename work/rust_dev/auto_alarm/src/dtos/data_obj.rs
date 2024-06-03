use crate::common::*;

pub trait AlarmDetail: Serialize {
    fn to_map(&self) -> HashMap<String, String>;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlarmDetailInfo {
    pub host_info: String,
    pub metric_type: String,
    pub metric_val: f64,
}

impl AlarmDetail for AlarmDetailInfo {
    
    fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        
        map.insert("host_info".to_string(), self.host_info.clone());
        map.insert(self.metric_type.to_string(), self.metric_val.to_string());
        
        map
    }

}

#[derive(Serialize, Deserialize, Debug)]
pub struct AlarmDetailError {
    pub err_content: String,
}

impl AlarmDetail for AlarmDetailError {

    fn to_map(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        
        map.insert(String::from("err_content"), self.err_content.to_string());

        map
    }

}


#[derive(Serialize, Deserialize, Debug)]
pub struct AlarmMetricForm<T> where T: AlarmDetail{
    pub alarm_type: String,
    pub monitor_type: String,
    pub cluster_name: String,
    pub kibana_url: String,
    pub contents: Vec<T>
}

impl<T> AlarmMetricForm<T> where T: AlarmDetail {}