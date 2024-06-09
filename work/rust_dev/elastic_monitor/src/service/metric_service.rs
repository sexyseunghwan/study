use crate::common::*;

use crate::service::es_service::*;
use crate::service::kafka_service::*;

use crate::dto::alarm_related_dtos::*;
use crate::dto::es_dtos::*;

use crate::utils_modules::time_utils::*;


/*
    Function to check whether allocatable shards exist in ES Cluster
*/
pub async fn get_shard_status(es_client: &EsHelper, cluster_name: &str, shard_limit: f64, kibana_url: &str, kafka_client: &ProduceBroker) -> Result<bool, anyhow::Error> {

    //let mut bool_flag = false;
    
    // ==============================================================
    // ========== 1. Get the value of max_shards_per_node. ==========
    // ==============================================================
    let cluster_settings_res = String::from("_cluster/settings?include_defaults=true");
    let max_shard_cnt_res = es_client.es_get_infos(cluster_settings_res.as_str(), 5).await?;

    let default_max_shards_per_node = 1000.0;
    let mut max_shards_per_node = default_max_shards_per_node;
    
    /*
        Setting value extraction and parsing function
    */
    fn extract_and_parse(value: Option<&str>, default: f64) -> f64 {
        match value {
            Some(val) => val.parse().unwrap_or(default),
            None => default,
        }
    }
    
    if let Some(cluster_settings) = max_shard_cnt_res.get("defaults").and_then(|d| d["cluster"]["max_shards_per_node"].as_str()) {
        max_shards_per_node = extract_and_parse(Some(cluster_settings), default_max_shards_per_node);
    }
    
    if let Some(cluster_settings) = max_shard_cnt_res.get("transient").and_then(|t| t["cluster"]["max_shards_per_node"].as_str()) {
        max_shards_per_node = extract_and_parse(Some(cluster_settings), default_max_shards_per_node);
    }

    if let Some(cluster_settings) = max_shard_cnt_res.get("persistent").and_then(|p| p["cluster"]["max_shards_per_node"].as_str()) {
        max_shards_per_node = extract_and_parse(Some(cluster_settings), default_max_shards_per_node);
    }
    
    
    // ===========================================================
    // ========== 2. Collects the number of data nodes. ==========
    // ===========================================================
    let shard_res = es_client.es_get_infos("_nodes/http", 5).await?;
    let mut data_node_cnt = 0.0;
    
    if let Some(node_map) = shard_res["nodes"].as_object() {
        
        for node_obj in node_map {

            let node_info = node_obj.1;
            
            if let Some(node_role_list) = node_info.get("roles").and_then(Value::as_array) {

                for node_role in node_role_list {
                    
                    if node_role.as_str() == Some("data") {
                        data_node_cnt += 1.0;
                    }
                }

            }
        }
    }
    
    // ======================================================================================
    // ========== 3. Collects the number of shards the cluster is currently using. ==========
    // ======================================================================================
    let using_shard_cnt_res = es_client.es_get_infos("_cluster/stats", 5).await?;
    let mut using_shard_cnt = 0.0;

    if let Some(res) = using_shard_cnt_res.get("indices").and_then(|t| t["shards"]["total"].as_f64()) {
        using_shard_cnt = res;
    } 
    
    let total_avail_shard_cnt = data_node_cnt * max_shards_per_node;
    let used_shard_per = round_to_two_decimal_places((using_shard_cnt / total_avail_shard_cnt) * 100.0);

    // ========================================================================================================================
    // ============================== 4. Monitoring information is produced in Monitoring Kafka. ==============================
    // ========================================================================================================================
    let cur_time_utc = get_current_utc_time("%Y-%m-%dT%H:%M:%S%.3fZ");

    // If there are enough shards that can be allocated to the ES cluster, the bool_flag value becomes TRUE.
    let exceed_flag = used_shard_per >= shard_limit;

    let monitor_metric_from = 
        MonitorMetricForm::new(cur_time_utc, String::from("ES"), cluster_name.to_string(), String::from(""), String::from("shard_usage"), used_shard_per, kibana_url.to_string(), exceed_flag);
    
    let monitor_metric_list = vec![monitor_metric_from];
    
    kafka_client.send_message_to_kafka_metric(&monitor_metric_list, "nosql_metric_log").await?;
    
    // if used_shard_per >= shard_limit {
    //     bool_flag = true;
        
    //     let msg_info = 
    //         AlarmDetailInfo::new(String::from(""),String::from("shard_usage"), used_shard_per);

    //     let msg_info_list: Vec<AlarmDetailInfo> = vec![msg_info];
        
    //     let alarm_info = 
    //         AlarmMetricForm::new(String::from("metric_alarm"), String::from("ES"), cluster_name.to_string(), kibana_url.to_string(), msg_info_list);

    //         kafka_client.send_message_to_kafka_alarm(&alarm_info, "nosql_mon_log").await?;
    // }

    Ok(exceed_flag)
}




/* 
    Function to monitor disk metrics of each cluster
*/
pub async fn get_es_disk_state(es_client: &EsHelper, kafka_client: &ProduceBroker, es_host_info: &ESMetricInfoExtend, disk_limit: f64, kibana_url: &str) -> Result<(), anyhow::Error> {

    let curr_utc_time = get_current_utc_time("%Y.%m.%d");
    let index_name = format!(".monitoring-es-{}-{}",es_host_info.system_version, curr_utc_time);
    let cluster_name = &es_host_info.cluster_name;

    let query = json!({
        "query": {
            "bool": {
            "filter": [
                {
                "range": {
                    "timestamp": {
                    "gte": "now-40s",
                    "lte": "now"
                    }
                }
                }
            ]
            }
        },
        "_source": ["aggregations"],
        "aggs": {
            "nodes": {
            "terms": {
                "field": "source_node.transport_address",
                "size": 100 
            },
            "aggs": {
                "total_disk": {
                "max": {
                    "field": "node_stats.fs.total.total_in_bytes"
                }
                },
                "available_disk": {
                "min": {
                    "field": "node_stats.fs.total.available_in_bytes"
                }
                }
            }
            }
        }
    });
    
    let disk_result = es_client.es_search(index_name.as_str(), query, 5).await?;
    
    let mut msg_json_list: Vec<AlarmDetailInfo> = Vec::new();

    if let Some(buckets) = disk_result["aggregations"]["nodes"]["buckets"].as_array() {
        
        // It contains information for EACH NODE in the cluster.
        for bucket in buckets {

            let host_ip_port = bucket["key"].as_str().unwrap_or("");
            let total_disk = bucket["total_disk"]["value"].as_f64().unwrap_or(-1.0);
            let avail_disk = bucket["available_disk"]["value"].as_f64().unwrap_or(-1.0);

            if host_ip_port.is_empty() || total_disk < 0.0 || avail_disk < 0.0 {
                error!("Invalid data encountered");
                continue;
            }
            
            let usage_disk_per = round_to_two_decimal_places(((total_disk - avail_disk) / total_disk) * 100.0);
            
            //info!("{:?}", usage_disk_per);

            // ============================== Monitoring information is produced in Monitoring Kafka. ==============================
            let cur_time_utc = get_current_utc_time("%Y-%m-%dT%H:%M:%S%.3fZ");
            let exceed_yn = usage_disk_per >= disk_limit;

            let monitor_metric_from = 
                MonitorMetricForm::new(cur_time_utc, String::from("ES"), cluster_name.clone(), host_ip_port.to_string(), String::from("disk_used"), usage_disk_per, kibana_url.to_string(), exceed_yn);
            
            let monitor_metric_list: Vec<MonitorMetricForm> = vec![monitor_metric_from];
            kafka_client.send_message_to_kafka_metric(&monitor_metric_list, "nosql_metric_log").await?;
            
            // [ Deprecated ]
            // if usage_disk_per >= disk_limit {
                
            //     let detail_infos = AlarmDetailInfo::new(host_ip_port.to_string(), String::from("disk_used"), usage_disk_per);
                
            //     msg_json_list.push(detail_infos)
            // }
        }//for
    }
    
    // Message for which an alarm should be sent.
    // if !msg_json_list.is_empty() {
        
    //     let msg_info = AlarmMetricForm::new(String::from("metric_alarm"), String::from("ES"), cluster_name.to_string(), kibana_url.to_string(), kibana_url.to_string(), msg_json_list);

    //     kafka_client.send_message_to_kafka_alarm(&msg_info, "nosql_mon_log").await?;
    // }
    
    Ok(())
}




/*
    Function to monitor cpu,jvm metrics of each cluster
*/
pub async fn get_es_jvm_cpu_state(es_client: &EsHelper, kafka_client: &ProduceBroker, es_host_info: &ESMetricInfoExtend, jvm_limit: f64, cpu_limit: f64, kibana_url: &str) -> Result<(), anyhow::Error> {

    let curr_utc_time = get_current_utc_time("%Y.%m.%d");
    let index_name = format!(".monitoring-es-{}-{}",es_host_info.system_version, curr_utc_time);
    let cluster_name = &es_host_info.cluster_name;
    
    let query = json!({
        "query": {
            "bool": {
            "filter": [
                {
                "range": {
                    "timestamp": {
                    "gte": "now-120s",
                    "lte": "now"
                    }
                }
                }
            ]
            }
        },
        "_source": ["aggregations"],
        "aggs": {
            "terms": {
                "terms": {
                    "field": "source_node.transport_address",
                    "size": 50
                },
                "aggs": {
                    "heap_avg": {
                    "avg": {
                        "field": "node_stats.jvm.mem.heap_used_percent"
                    }
                    },
                    "cpu_avg": {
                    "avg": {
                        "field": "node_stats.process.cpu.percent"
                    }
                    },
                    "node_name": {
                    "terms": {
                        "field": "source_node.name",
                        "size": 50
                    }
                    }
                }
            }
        }
    });
    
    let cpu_jvm_result = es_client.es_search(index_name.as_str(), query, 5).await?;

    let mut msg_json_list: Vec<AlarmDetailInfo> = Vec::new();

    if let Some(buckets) = cpu_jvm_result["aggregations"]["terms"]["buckets"].as_array() {
        
        for bucket in buckets {
            
            let host_ip_port = bucket["key"].as_str().unwrap_or("");
            let mut use_cpu = bucket["cpu_avg"]["value"].as_f64().unwrap_or(-1.0);
            let mut use_jvm = bucket["heap_avg"]["value"].as_f64().unwrap_or(-1.0);

            if host_ip_port.is_empty() || use_cpu < 0.0 || use_jvm < 0.0 {
                error!("Invalid data encountered");
                continue;
            }
            
            use_cpu = round_to_two_decimal_places(use_cpu);
            use_jvm = round_to_two_decimal_places(use_jvm);
            
            // Monitoring information is produced in Monitoring Kafka. ==============================
            let mut monitor_metric_list: Vec<MonitorMetricForm> = Vec::new();
            let cur_time_utc = get_current_utc_time("%Y-%m-%dT%H:%M:%S%.3fZ");
            
            let cpu_exceed_yn = use_cpu >= cpu_limit;
            let jvm_exceed_yn = use_jvm >= jvm_limit;

            // cpu metric
            let monitor_metric_from_cpu = 
                MonitorMetricForm::new(cur_time_utc.clone(), String::from("ES"), cluster_name.clone(), host_ip_port.to_string(), String::from("cpu_used_avg"), use_cpu, kibana_url.to_string(), cpu_exceed_yn);
            
            // jvm metric
            let monitor_metric_from_jvm = 
                MonitorMetricForm::new(cur_time_utc.clone(), String::from("ES"), cluster_name.clone(), host_ip_port.to_string(), String::from("jvm_used_avg"), use_jvm, kibana_url.to_string(), jvm_exceed_yn);
            
                        
            monitor_metric_list.push(monitor_metric_from_cpu);
            monitor_metric_list.push(monitor_metric_from_jvm);

            kafka_client.send_message_to_kafka_metric(&monitor_metric_list, "nosql_metric_log").await?;
            
            // if use_jvm >= jvm_limit {
                
            //     let detail_infos = 
            //         AlarmDetailInfo::new(host_ip_port.to_string(), String::from("jvm_used_avg"), use_jvm); 
                
            //     msg_json_list.push(detail_infos)
            // }

            // if use_cpu >= cpu_limit {
                
            //     let detail_infos = 
            //         AlarmDetailInfo::new(host_ip_port.to_string(), String::from("cpu_used_avg"), use_cpu);
                
            //     msg_json_list.push(detail_infos)
            // }
            
        }
    }
    
    // Message for which an alarm should be sent.
    if !msg_json_list.is_empty() {
        
        let msg_info = AlarmMetricForm::new(String::from("metric_alarm"), String::from("ES"), cluster_name.to_string(), kibana_url.to_string(), msg_json_list);

        kafka_client.send_message_to_kafka_alarm(&msg_info, "nosql_mon_log").await?;
    }
    
    Ok(())
}