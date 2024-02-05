use anyhow::Ok;
use kvserver::KVClient;
use tarpc::{client, context, tokio_serde::formats::Json};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let server =
        tarpc::serde_transport::tcp::connect("localhost:9010", Json::default).await?;
    let client = KVClient::new(client::Config::default(), server).spawn();

    let ctx = context::current();
    println!("{:?}", client.put(ctx, "lol".to_string(), "lol".to_string()).await?);
    println!("{:?}", client.get(ctx, "lol".to_string()).await?);
    println!("{:?}", client.append(ctx, "lol".to_string(), "lol2".to_string()).await?);
    println!("{:?}", client.get(ctx, "lol".to_string()).await?);
    Ok(())
}
