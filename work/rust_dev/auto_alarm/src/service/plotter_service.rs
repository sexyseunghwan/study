use crate::common::*;

#[derive(Debug)]
pub struct MetricObject {
    pub metric_name: String, 
    pub metric_data_set_list: Vec<(DateTime<Utc>,i32)>, 
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