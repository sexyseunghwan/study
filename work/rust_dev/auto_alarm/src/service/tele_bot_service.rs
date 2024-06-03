use crate::common::*;
use crate::service::mysql_async_service::*;
use crate::dtos::data_obj::*;

#[derive(Clone, Serialize, Deserialize, FromRow, Debug)]
pub struct Telebot {
    pub bot_token: String,
    pub chat_room_id: String,
}


/*
    List of Telegram bot information to share.
*/
pub struct SharedResource {
    pub tele_data: Arc<RwLock<Vec<Telebot>>>
}

/* 
    Function that periodically updates telegram bot information
*/
pub async fn update_telegram_data(shared_res: Arc<SharedResource>, mysql_client: MySqlAsyncClient, timer: u64){
    
    let mut interval = tokio::time::interval(Duration::from_secs(timer));
    
    loop {
        
        interval.tick().await;
        
        let query = 
            r"
            SELECT 
                bot_token
            ,   chat_room_id
            FROM NOSQL_MON_BOT
            WHERE mon_apply_yn = ?";
        
        match mysql_client.query_select_from_param(query, (1,)).await {
            Ok(new_data) => {
                let mut telegram_data = shared_res.tele_data.write().await;
                *telegram_data = new_data;
            }, 
            Err(err) => {
                error!("{:?}", err);
            }
        }
    }
}


impl Telebot {
    
    /*
        Function that processes messages received from Kafka
    */
    pub async fn process_msg<T: AlarmDetail>(&self, alarm_msg: &AlarmMetricForm<T>) -> Result<(), anyhow::Error> {

        let mut msg_contents = String::new();   
        let mut process_yn = false;

        
        if alarm_msg.alarm_type == "metric_alarm" {

            msg_contents.push_str(format!("==== Metric Alert {}====\n", alarm_msg.monitor_type).as_ref());
            msg_contents.push_str(format!("[{}]\n", alarm_msg.cluster_name).as_ref());
            
            if alarm_msg.monitor_type == "ES" {
                msg_contents.push_str(format!("[kibana url] {}\n\n", alarm_msg.kibana_url).as_ref());
            }
            
            process_yn = true;
            
            let metrics_mapping = [
                ("host_info", "host_info: {}\n"),
                ("disk_used", "- disk used: {}%\n"),
                ("cpu_used_avg", "- cpu used: {}%\n"),
                ("jvm_used_avg", "- jvm used: {}%\n"),
                ("memory_used", "- memory_used: {}%\n"),
                ("cache_hit_per", "- cache_hit_per: {}%\n"),
                ("mem_fragmentation_ratio", "- mem_frag_ratio: {}\n"),
                ("mem_fragmentation_bytes", "- mem_frag_bytes: {}\n"),
                ("shard_usage", "- shard_usage_per: {}%\n"),
                ("memory_used_avg", "- memory_used_per: {}%\n"),
                ("m_s_sync", "- master_slave_sync_sec: {} sec ago\n")
            ];
            
            for alarm_detail_info in &alarm_msg.contents {

                let alarm_detail_map = alarm_detail_info.to_map();
                
                for (key, format_str) in metrics_mapping {
                    
                    if alarm_detail_map.contains_key(key) {
                        let metrics = format_str.replace("{}", alarm_detail_map[key].as_str());
                        msg_contents.push_str(&metrics);
                    }
                } 
            } // for
        

        } else if alarm_msg.alarm_type == "error_alarm"  {

            msg_contents.push_str(format!("==== Error Alert {}====\n", alarm_msg.monitor_type).as_ref());
            msg_contents.push_str(format!("[{}]\n", alarm_msg.cluster_name).as_ref());

            if alarm_msg.monitor_type == "ES" {
                msg_contents.push_str(format!("[kibana url] {}\n\n", alarm_msg.kibana_url).as_ref());
            }
            
            process_yn = true;

            let metrics_mapping = [
                ("err_content", "{}\n")
            ];
            
            for alarm_detail_info in &alarm_msg.contents {

                let alarm_detail_map = alarm_detail_info.to_map();
                
                for (key, format_str) in metrics_mapping {
                    
                    if alarm_detail_map.contains_key(key) {
                        let metrics = format_str.replace("{}", alarm_detail_map[key].as_str());
                        msg_contents.push_str(&metrics);
                    }
                } 
            } // for
        } 
        
        if process_yn {

            match self.bot_send(&msg_contents).await {
                Ok(_) => (),
                Err(err) => {
                    error!("{:?}", err);
                }
            }
        }
        
        Ok(())

    }

    
    /*
        Functions that send messages through telegram bot
    */
    pub async fn bot_send(&self, send_msg: &str) -> Result<(), anyhow::Error> {
        
        let mut try_cnt = 0;

        while try_cnt < 3 {

            let url = format!(
                "https://api.telegram.org/bot{}/sendMessage",
                self.bot_token
            );

            let body = serde_json::json!({
                "chat_id": self.chat_room_id,
                "text": send_msg
            });

            let client = reqwest::Client::new();
            let res = client.post(&url)
                .header("Content-Type", "application/json")
                .body(body.to_string())
                .send()
                .await;

            match res {
                Ok(res) => {                    
                    if !res.status().is_success() {
                        let err_text = res.text().await.unwrap_or_else(|_| "Failed to get error message".to_string());
                        error!("Failed to send message: {}. http communication retry begins after 40 seconds.", err_text);
                        try_cnt += 1;
                        thread::sleep(Duration::from_secs(40));
                    } else {
                        info!("Success send message");
                        break;
                    }
                },
                Err(err) => {
                    error!("HTTP request failed: {}. http communication retry begins after 40 seconds.", err);
                    try_cnt += 1;
                    thread::sleep(Duration::from_secs(40));
                }
            }
        }

        if try_cnt == 3 {
            return Err(anyhow!("The system attempted to communicate with the telegram bot more than 3 times, but failed.".to_string()));
        }
        
        Ok(())
    }

}