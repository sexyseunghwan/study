use crate::common::*;

use crate::dicontainer::*;

use crate::configure::mysql_config::*;
use crate::configure::diesel_config::*;
use crate::configure::redis_config::*;
use crate::configure::kafka_config::*;

use crate::controller::test_controller;
use crate::controller::auth_controller;

use crate::util_mod::jwt_utils::*;

use super::pic_controller;


/*
    Function that initializes the db connection pool.
*/
fn initiate_db_conn_pool() -> Result<web::Data<DbState>, anyhow::Error> {

    // 1. Classic Mysql Connection Pool
    let mysql_classic_pool: Pool = init_rdb_conn()?; 

    // 2. Diesel Mysql Connection Pool (ORM)
    let mysql_diesel_pool: diesel::r2d2::Pool<ConnectionManager<MysqlConnection>> = init_rdb_diesel_conn()?;

    // 3. Redis Connection
    let redis_conn = init_redis_conn()?;
    
    // 4. Kafka Connection
    let kafka_conn = init_kafka_conn()?;
    
    // Register each DB connection in [web::Data].ㄴ
    let app_state: web::Data<DbState> = web::Data::new(DbState {
        mysql_classic_pool,
        mysql_diesel_pool,
        redis_conn,
        kafka_conn
    });
    
    Ok(app_state)
}


/*
    Function that sets HTTPS.
*/
fn initiate_ssh_config() -> openssl::ssl::SslAcceptorBuilder {
    
    let ssh_val = env::var("SSH_ENV").expect("'SSH_ENV' must be set");

    let mut builder: openssl::ssl::SslAcceptorBuilder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    let (ssh_key_path, ssh_cert_path) = if ssh_val == "intel" {
        ("./https_intel/key.pem", "./https_intel/cert.pem")
    } else {
        ("./https_m1/key.pem", "./https_m1/cert.pem")
    };
    
    // SSL-related settings (to use HTTPS communication)
    builder
        .set_private_key_file(ssh_key_path, SslFiletype::PEM)
        .unwrap();
    builder
        .set_certificate_chain_file(ssh_cert_path)
        .unwrap();

    builder
}


/*
    Functions to bind multiple controllers
*/
pub async fn bind_controller() -> Result<(), anyhow::Error>
{
    
    // Initialize service-related global variables
    dotenv().ok();

    // Initiate HTTPS Configuration
    let builder = initiate_ssh_config();
    
    // Declare jwt secret keys globally.
    let _ = init_jwt_private_key().await;
    
    // Initiate DB Connection Pool - MySQL, Redis, Kafka
    let db_pools: web::Data<DbState> = initiate_db_conn_pool()?;
    
    // Add to DI Container.
    let di_container = web::Data::new(DIContainer::new(db_pools));
    
    infos("Web-Service start successfully").await;
    
    // Initializes the SERVICE configuration.
    let _ = HttpServer::new(move || {
        
        let cors = Cors::default()
            .allowed_origin("https://localhost:3000")
            .allowed_origin("https://127.0.0.1:3000")
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600); // Cache the results of the pre-request (preflight) for 1 hour
            //.allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT])

        App::new()
            .wrap(cors)
            .app_data(di_container.clone())
            .configure(test_controller::config) 
            .configure(auth_controller::config) 
            .configure(pic_controller::config) 
    })
    .bind_openssl("127.0.0.1:8080", builder)? 
    .run()
    .await;
    
    Ok(())
}