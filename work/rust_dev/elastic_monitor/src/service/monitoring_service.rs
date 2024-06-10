use crate::common::*;

use crate::service::kafka_service::*;
use crate::service::es_service::*;
use crate::service::metric_service::*;
use crate::service::mysql_async_service::*;

use crate::dto::aes_dtos::*;
use crate::dto::es_dtos::*;
use crate::dto::alarm_related_dtos::*;


use crate::utils_modules::time_utils::*;
use crate::utils_modules::dec_utils::*;
use crate::utils_modules::parsing_utils::*;

/*
    Considering the system monitoring index creation time, monitoring is stopped while the system index is being created.
*/
pub fn monitor_thread_stop() {
    
    let m_date = get_curr_utc_time_naive();

    if m_date.hour() == 0 && m_date.second() <= 3 {
        thread::sleep(Duration::from_secs(180));
    }
}

/* 
    For multithreaded programming, it creates a vector of elasticsearch-related vectors.
    
    thread1 -> [nd.log, nd.elk.logrecord, ...] => monitoring
    thread2 -> [total_search, listing_search, ...] => monitoring
    thread3 -> [catalog_listing, listing_data, ...] => monitoring
    ...
*/
fn get_multi_thread_es_vec(host_info_list: Vec<ESMetricInfoExtend>) -> Result<Vec<Vec<ESMetricInfoExtend>>, anyhow::Error> {

    let mut thread_cluster_list: Vec<Vec<ESMetricInfoExtend>> = Vec::new();

    let cluster_list_size = host_info_list.len();
    
    // Source code that determines how many THREADS to RUN in total.
    let thread_cnt = match cluster_list_size {
        0..=7 => if cluster_list_size % 2 == 0 { 2 } else { 3 },
        _ => if cluster_list_size % 2 == 0 { 4 } else { 3 },
    };
    
    let size_q = cluster_list_size / thread_cnt;
    let size_r = cluster_list_size % thread_cnt;
    let mut clster_idx = 0;
    
    for i in 0..thread_cnt {

        let mut es_list: Vec<ESMetricInfoExtend> = Vec::new();
        
        if i == thread_cnt - 1 {
            
            for item in host_info_list.iter().skip(clster_idx).take(size_q + size_r) {
                es_list.push(item.clone());
            }
        
        } else {

            for item in host_info_list.iter().skip(clster_idx).take(size_q) {
                es_list.push(item.clone());
            }
            
            clster_idx += size_q;
        }

        thread_cluster_list.push(es_list);
    }
    
    Ok(thread_cluster_list)

}



/* 
    Function that performs monitoring through multi-thread programming     
*/
async fn multi_es_monitor(host_info_list: Vec<ESMetricInfoExtend>, kafka_client: ProduceBroker) -> Result<(), anyhow::Error> {
    
    for es_cluster in host_info_list {

        // ES connection object for which MONITORING will be performed
        let es_host_client = match EsHelper::new(&es_cluster).await {
            Ok(res) => res,
            Err(err) => {
                //error!("{:?}",err);
                //let error_msg = AlarmDetailError::new(es_cluster.cluster_name().to_string(), es_cluster.kibana_url().to_string(), err.to_string());
                //let detail_infos: AlarmMetricForm<AlarmDetailError> = AlarmMetricForm::new(String::from("error_alarm"), String::from("ES"), es_cluster.cluster_name, es_cluster.kibana_url, alarm_error_list);
                
                kafka_client.send_message_to_kafka_alarm(&error_msg, "nosql_err_log").await?;
                continue;
            }
        };
        
        let shard_check = get_shard_status(&es_host_client, &es_cluster.cluster_name, es_cluster.shard_limit, es_cluster.kibana_url(), &kafka_client).await?;
        
        /*
            If there is a problem with the shard allowance.
            Returns the True value if there is a problem with the shard.
        */
        if shard_check {
            continue;
        } 
        
        match get_es_disk_state(&es_host_client, &kafka_client, &es_cluster, es_cluster.disk_limit, es_cluster.kibana_url()).await {
            Ok(_) => {},
            Err(err) => { error!("get_es_disk_state ERROR : {:?}",err )}
        }
        
        match get_es_jvm_cpu_state(&es_host_client, &kafka_client, &es_cluster, es_cluster.jvm_limit, es_cluster.cpu_limit, es_cluster.kibana_url()).await {
            Ok(_) => {},
            Err(err) => { error!("get_es_jvm_cpu_state ERROR : {:?}",err )}
        }
        
    }
    
    Ok(())
}


/* 
    Function that monitors the indicator values of all ES clusters.
    All error handling is done in that function.
*/
pub async fn metric_monitor(kafka_client: &ProduceBroker, mysql_client: &MySqlAsyncClient, aes_infos: &AesInfosDTO) -> Result<(), anyhow::Error> {
    
    /*
        At the time of index generation, the thread is stopped.
    */
    monitor_thread_stop();
    
    /*
        ============================================================================================
        =========== 1) Collects all information on the ES Cluster subject to monitoring. ===========
        ============================================================================================
    */
    /* 
        Instance of nosql cluster infos (MySQL Object)
        => {cluster_name, user_id, user_pw_enc, system_version, ssl_option, shard_limit, disk_limit, cpu_limit, jvm_limit} 
    */
    let sql_res_cluster_infos: Vec<ESMetricInfo> = 
        mysql_client.query_select_from_param(
            r"
            SELECT 
                nct.cluster_name
            ,   IFNULL((MAX(nct.kibana_url)), 'NONE') AS kibana_url
            ,	MAX(nct.user_id) AS user_id
            ,	MAX(nct.user_pw_enc) AS user_pw_enc
            ,	MAX(nct.system_version) AS system_version
            ,	MAX(nct.ssl_option) AS ssl_option	
            ,	MAX(CASE WHEN nlm.metric_type = 'shard' THEN nlm.limit_value ELSE 0 END) AS shard_limit
            ,	MAX(CASE WHEN nlm.metric_type = 'disk' THEN nlm.limit_value ELSE 0 END) AS disk_limit
            ,	MAX(CASE WHEN nlm.metric_type = 'cpu' THEN nlm.limit_value ELSE 0 END) AS cpu_limit
            ,	MAX(CASE WHEN nlm.metric_type = 'jvm' THEN nlm.limit_value ELSE 0 END) AS jvm_limit
            FROM NOSQL_CLUSTER_TYPES nct
            INNER JOIN NOSQL_MON_GROUP nmg ON nct.group_name = nmg.group_name
            INNER JOIN NOSQL_LIMIT_METRICS nlm ON nlm.metric_name = nmg.metric_name
            WHERE nct.system_type = ?
            GROUP BY nct.cluster_name",
        ("ES", )).await?; 
    
    // Elasticsearch information vector to be monitored
    let mut host_info_list: Vec<ESMetricInfoExtend> = Vec::new();
    
    for host_types in sql_res_cluster_infos {
        
        let clust_name = host_types.cluster_name();

        // ES user_id
        let parsing_id = match host_types.user_id {
            Some(ref user_id) => user_id,
            None => ""
        };
        
        // ES user_pw -> decrypt
        let parsing_pw = match host_types.user_pw_enc {
            Some(ref user_pw_enc) => {
                
                if user_pw_enc.is_empty() {
                    String::from("")
                } else {
                     
                    let dec_pw = match decrypt(user_pw_enc, &aes_infos.aes_key, &aes_infos.aes_iv) {
                        Ok(dec_pw) => dec_pw,
                        Err(err) => {
                            error!("{:?}", err);
                            return Err(anyhow!("Encrypted data has failed to decrypt."));
                        }
                    };

                    let dec_pw_str = String::from_utf8_lossy(&dec_pw).to_string();
                    get_url_encoding(dec_pw_str.as_str())
                }
            },
            None => String::from("")
        };

        /* 
            Instance of NOSQL_HOST_INFO (MySQL Table)
            => {host_ip, host_port}
        */
        let node_host_infos: Vec<String> = 
            mysql_client.query_select_from_param(
                r"
                    SELECT 
                        CONCAT(host_ip, ':', host_port) as host_info 
                    FROM NOSQL_HOST_INFO 
                    WHERE system_type = ? 
                    AND cluster_name = ?", 
                ("ES", host_types.cluster_name())
            ).await?;

        let target_mon_es_obj = 
            ESMetricInfoExtend::new(clust_name.clone(), host_types.kibana_url,parsing_id.to_string(), parsing_pw,
                                        host_types.system_version, host_types.ssl_option, host_types.shard_limit,
                                        host_types.disk_limit, host_types.cpu_limit, host_types.jvm_limit, node_host_infos
                                        ); 

        host_info_list.push(target_mon_es_obj);
    }
    

    /* 
        ===============================================================================
        =========== 2) Accesses each ES cluster and collects metric values. ===========
        ===============================================================================
    */
    let multi_lists = get_multi_thread_es_vec(host_info_list)?;
    let mut handles: Vec<tokio::task::JoinHandle<()>> = Vec::new();

    for metric_obj in multi_lists {

        let kafka_client_clone = kafka_client.clone();

        let handle = tokio::spawn(async move {
            match multi_es_monitor(metric_obj.clone(), kafka_client_clone).await {
                Ok(_) => (),
                Err(err) => {
                    error!("{:?}", err);
                }
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.await?;
    }
    
    
    Ok(())
}
   