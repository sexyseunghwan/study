use crate::common::*;


#[derive(Serialize, Deserialize, Debug, Getters, Setters, new)]
pub struct MonitorMetricForm {
    pub time_stamp: String,
    pub monitor_type: String,
    pub cluster_name: String,
    pub host_info: String,
    pub metric_type: String,
    pub metric_value: f64,
}


/*
    ====================================================================== 
    =========== Alarm message-related structure template. ===========
    ======================================================================   
*/
pub trait AlarmDetail: Serialize {}

#[derive(Serialize, Deserialize, Debug, Getters, Setters, new)]
#[getset(get = "pub")]
pub struct AlarmDetailInfo {
    pub host_info: String,
    pub metric_type: String,
    pub metric_val: f64
}

impl AlarmDetail for AlarmDetailInfo {}


#[derive(Serialize, Deserialize, Debug, Getters, Setters, new)]
#[getset(get = "pub")]
pub struct AlarmDetailError {
    pub err_content: String,
}

impl AlarmDetail for AlarmDetailError {}


#[derive(Serialize, Deserialize, Debug, Getters, Setters, new)]
#[getset(get = "pub")]
pub struct AlarmMetricForm<T> where T: AlarmDetail{
    pub alarm_type: String,
    pub monitor_type: String,
    pub cluster_name: String,
    pub kibana_url: String,
    pub contents: Vec<T>
}

