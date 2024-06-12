pub use std::io::{Read, Write};
pub use std::thread;
pub use std::time::Instant;
pub use std::env;
pub use std::sync::{Arc, Mutex};

pub use mysql_async::Pool;
pub use mysql_async::prelude::*;

pub use log::{info, error};

pub use flexi_logger::{Logger, FileSpec, Criterion, Age, Naming, Cleanup, Record};

pub use chrono::{DateTime, Utc, NaiveDateTime, Timelike};

pub use serde::{Serialize, Deserialize};
pub use serde_json::{json, Value};

pub use futures::stream::StreamExt;

pub use reqwest;
pub use reqwest::Method;

pub use rdkafka::config::ClientConfig;
pub use rdkafka::consumer::Consumer;
pub use rdkafka::producer::{FutureProducer, FutureRecord};
pub use rdkafka::message::Message;

pub use anyhow::{Result, anyhow};

pub use dotenv::dotenv;

pub use rand::Rng; 
pub use rand::{SeedableRng, rngs::StdRng};

pub use openssl::symm::{Cipher, Crypter, Mode};

pub use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

pub use getset::{Getters, Setters, MutGetters};

pub use derive_new::new;

pub use tokio::time::{timeout, Duration};

pub use once_cell::sync::Lazy;

use crate::service::kafka_service::*;


// Kafka producer to MANAGE logging
pub static LOGGER_PRODUCER: Lazy<Arc<Mutex<ProduceBroker>>> = Lazy::new(|| {
    
    let kafka_host = env::var("KAFKA_HOST").expect("'KAFKA_HOST' must be set");
    let producer = ProduceBroker::new(&kafka_host).expect("ProduceBroker creation failed");
    
    Arc::new(Mutex::new(producer))
});

#[derive(Debug, Getters, Clone, new)]
#[getset(get = "pub")]
pub struct Testing {
    pub name: String
}

pub static TEST_VAL: Lazy<Arc<Mutex<Testing>>> = Lazy::new(|| {
    
    let test = Testing::new(String::from("test"));

    Arc::new(Mutex::new(test))
});
