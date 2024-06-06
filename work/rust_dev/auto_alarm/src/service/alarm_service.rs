use crate::common::*;

use crate::service::kafka_service::*;
use crate::service::tele_bot_service::*;
use crate::service::mysql_async_service::*;
use crate::service::es_service::*;

/*
    If an error occurs among the ES subject to monitoring, A function that sends abnormal situations through Telegram bot.
*/
pub async fn push_alarm_to_telebot(kafka_client: KafkaBroker, mysql_client: MySqlAsyncClient, es_client: EsHelper) -> Result<(), anyhow::Error> {
    
    let sql_query = 
        r"
        SELECT 
            bot_token
        ,   chat_room_id
        FROM NOSQL_MON_BOT
        WHERE mon_apply_yn = ?
        AND role_type = ?";
    
    let tele_bot_list: Vec<Telebot> = 
        mysql_client.query_select_from_param(
            sql_query, (1,1,)
        ).await?;
    
    let shared_res = Arc::new(SharedResource {
        tele_data: Arc::new(RwLock::new(tele_bot_list)),
    });
    
    let shared_res_clone_handle1 = Arc::clone(&shared_res);
    let shared_res_clone_handle2 = Arc::clone(&shared_res);
    
    // It periodically updates telegram information.
    let handle1 = spawn(async move {
        update_telegram_data(shared_res_clone_handle1, mysql_client, 30).await;
    });
    
    // The error log accumulated in kafka is sent a message through telegram bot.
    let handle2 = spawn(async move {
        let _ = kafka_client.consume_and_send_messages("nosql_mon_log", es_client, shared_res_clone_handle2).await;
    });
    
    handle1.await?;
    handle2.await?;

    Ok(())
}