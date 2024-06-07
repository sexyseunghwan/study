use crate::common::*;

#[derive(Debug, new)]
pub struct MetricObject {
    pub cluster_name: String,
    pub metric_type: String, 
    pub metric_data_set_list: Vec<(DateTime<Utc>,f64)>, 
}

#[derive(Debug)]
pub struct PlotterStruct {
    pub out_file_name: String, 
    pub metric_obj_list: Vec<MetricObject>, 
    pub time_slice: i32, 
}

/* */
impl PlotterStruct {
    
    //
    
    
}