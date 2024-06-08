use crate::common::*;

#[derive(Debug, new)]
pub struct MetricObject {
    pub cluster_name: String,
    pub metric_type: String, 
    pub metric_data_set_list: Vec<(DateTime<Utc>,f64)>,
    pub msg_contents: String
}

#[derive(Debug, new)]
pub struct PlotterStruct {
    pub out_file_name: String, 
    pub metric_obj: MetricObject, 
    pub time_slice: i32, 
}

/* */
impl PlotterStruct {
    
    //
    
    
}