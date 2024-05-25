pub use std::io::{Write, BufReader};
pub use std::sync::{Arc, Mutex};
pub use std::env;
pub use std::fmt::Debug;
pub use std::time::{Instant, Duration};
pub use std::collections::HashMap;
pub use std::time::{SystemTime, UNIX_EPOCH};
pub use std::task::{Context, Poll};
pub use std::future::Future;
pub use std::pin::Pin;


pub use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Responder, cookie::{Cookie, SameSite}, middleware, web::Json, Error, error::ErrorUnauthorized, HttpMessage, dev::ServiceRequest, dev::ServiceResponse};
pub use actix_service::{Service, Transform};

pub use actix_cors::Cors;

pub use tokio::task;
pub use tokio::sync::RwLock;

pub use dotenv::dotenv;

pub use log::{info, error};

pub use flexi_logger::{Logger, FileSpec, Criterion, Age, Naming, Cleanup, Record};
//pub use elasticsearch::params::Sort::Duration;
pub use flexi_logger::writers::FileLogWriter;

pub use futures::future::{ok, Ready};

pub use chrono::{Local, DateTime, Utc, NaiveDate, Datelike, NaiveDateTime, TimeZone};

pub use anyhow::{Result, anyhow};

pub use serde::{Serialize, Deserialize, de::DeserializeOwned};
pub use serde_json::{json, Value};
pub use serde_json::Error as jsonError;


pub use getset::{Getters, Setters, MutGetters, CopyGetters};

pub use derive_new::new;

pub use reqwest;
pub use reqwest::{Client, Method, Response, Error as ReqwestError};

pub use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

pub use bcrypt::{hash, verify, DEFAULT_COST};

pub use mysql_async::{Pool, Row};
pub use mysql_async::prelude::*;

pub use once_cell::sync::Lazy;

pub use async_trait::async_trait;

pub use diesel::mysql::MysqlConnection;
pub use diesel::prelude::*;
pub use diesel::r2d2::{ConnectionManager};
pub use diesel::sql_types::*;

pub use redis::{Commands, AsyncCommands};
pub use redis::cluster::ClusterClient;

pub use rdkafka::producer::{FutureProducer,FutureRecord};
pub use rdkafka::ClientConfig;

pub use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation, Algorithm};

pub use regex::Regex;

/* ============================== static variable ============================== */
// Global variables related to configuration
// pub static SSH_ENV: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::new()));
// pub static DATABASE_URL: Lazy<RwLock<String>> = Lazy::new(|| RwLock::new(String::new()));

pub static ACCESS_KEY: Lazy<Arc<RwLock<String>>> = Lazy::new(|| Arc::new(RwLock::new(String::new())));
pub static REFRESH_KEY: Lazy<Arc<RwLock<String>>> = Lazy::new(|| Arc::new(RwLock::new(String::new())));

pub struct DbState {
    pub mysql_classic_pool: Pool,                                           
    pub mysql_diesel_pool: diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>,
    pub redis_conn: ClusterClient,
    pub kafka_conn: crate::configure::kafka_config::ProduceBroker
}
///
// ======== DEPRECATED...? ========
// Global variables related to MySQL connection
//pub static RDB_CONN_POOL: Lazy<RwLock<Option<Pool>>> = Lazy::new(|| RwLock::new(None));

// Global variables related to MySQL connection(ORM)
//type DbPool = diesel::r2d2::Pool<ConnectionManager<MysqlConnection>>;
//pub static RDB_CONN_DIESEL_POOL: Lazy<RwLock<Option<DbPool>>> = Lazy::new(|| RwLock::new(None));

pub use crate::util_mod::logger_utils::errors;
pub use crate::util_mod::logger_utils::infos;
