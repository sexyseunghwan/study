    use crate::common::*;

use crate::service::es_service::*;



/*
    Function that checks if the current disk usage exceeds the threshold
*/
async fn check_disk_usage(es_helper: &EsHelper) -> Result<bool, anyhow::Error> {

    let metric_info_list = es_helper.cluster_stats_fs().await?;
    let mut del_flag = false;

    for metric_info in metric_info_list {

        let total_in_bytes = *metric_info.total_in_bytes() as f64;
        let available_in_bytes = *metric_info.available_in_bytes() as f64;

        let use_in_byte = total_in_bytes - available_in_bytes;
        let use_in_byte_per = (use_in_byte / total_in_bytes) * 100.0;
        
        info!("{} : {}", metric_info.node_host(), use_in_byte_per);

        if use_in_byte_per > 70.0 { 
            del_flag = true; 
            break;
        }
    }  

    Ok(del_flag)
}


/*
    Function to DELETE the largest index (the system index is not erased)
*/
pub async fn clear_service(es_helper: &EsHelper) -> Result<(), anyhow::Error> {
    
    let mut loop_flag = check_disk_usage(es_helper).await?;
    
    while loop_flag {
        
        let mut index_infos_queue = es_helper.cluster_cat_query("store.size:desc", 5).await?;
        
        while let Some(index_info) = index_infos_queue.pop_front() {

            let index_name = index_info.index_name();
            
            // ============!!!!!! [ DANGER ] !!!!!!============
            // Here, the index with the largest capacity is erased.
            es_helper.cluster_delete_query(index_name).await?;
            
            if ! check_disk_usage(es_helper).await? {
                loop_flag = false;
                break;
            }
        }
    }  

   Ok(()) 
}