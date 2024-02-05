use std::{collections::HashMap, sync::{Arc, Mutex}};

use anyhow::Ok;
use futures::{future, prelude::*};
use kvserver::KV;
use tarpc::{
    context,
    server::{incoming::Incoming, BaseChannel},
    tokio_serde::formats::Json,
};

#[derive(Clone)]
struct KVServer {
    data: Arc<Mutex<HashMap<String, String>>>,
}

#[tarpc::server]
impl KV for KVServer {
    async fn put(self, _: context::Context, key: String, value: String) -> String {
        self.data.lock().unwrap().insert(key, value);
        return "Value Added".to_string();
    }
    async fn append(self, _: context::Context, key: String, value: String) -> String {
        let mut data = self.data.lock().unwrap();
        if data.contains_key(&key) {
            let msg = format!("Appended Key, Old Value: {}", data.get(&key).unwrap());
            data.remove(&key);
            data.insert(key, value);
            return msg;
        }
        return "Error: Key does not exist".to_string();
    }
    async fn get(self, _: context::Context, key: String) -> String {
        let data = self.data.lock().unwrap();
        if data.contains_key(&key) {
            return format!("Value: {}", data.get(&key).unwrap());
        }
        return "Error: Key does not exist".to_string();
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server = KVServer { 
        data: Arc::new(Mutex::new(HashMap::new())),
    };
    let add_listener = tarpc::serde_transport::tcp::listen("localhost:9010", Json::default)
        .await?
        .filter_map(|r| future::ready(r.ok()));
    let add_server = add_listener
        .map(BaseChannel::with_defaults)
        .execute(server.serve());
    let j = tokio::spawn(add_server);

    j.await?;

    Ok(())
}
