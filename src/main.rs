
  
use fred::prelude::*;
use futures::stream::StreamExt;
use std::time::Duration;
use tokio::time::sleep;
use serde_json;
use serde_json::Value;



async fn set_connection()  -> RedisClient{
  let server = ServerConfig::Centralized{
    host: "localhost".to_string(),
    port: 6379,
  };
  let mut config = RedisConfig::default();
  config.server = server;
  let policy = ReconnectPolicy::new_linear(0, 5000, 500);
  let subscriber_client = RedisClient::new(config);

  let _ = tokio::spawn(subscriber_client.on_error().for_each(|e| async move {
    println!("Subscriber client connection error: {:?}", e);
  }));
  let _ = tokio::spawn(subscriber_client.on_reconnect().for_each(|client| async move {
    println!("Subscriber client reconnected.");
    if let Err(e) = client.subscribe("market").await {
      println!("Error resubscribing: {:?}", e);
    }
  }));

  let _ = subscriber_client.connect(Some(policy));
  let _ = subscriber_client.wait_for_connect().await;
  subscriber_client
}


#[tokio::main]
async fn main() -> Result<(), RedisError> {
  pretty_env_logger::init();
  let  client = set_connection().await;

  let subscribe_task = tokio::spawn(async move {
    let mut message_stream = client.on_message();

    while let Some((channel, message)) = message_stream.next().await {
      match message{
        fred::types::RedisValue::String(_val) =>{
          let val: Value = serde_json::from_str(&_val).unwrap();
          println!("{:?}", val["type"].as_str().unwrap());
        },
        _ => {
          println!("{:?}", message);
        }
      }
      // let v: Value = serde_json::from_str(message);
    }
    Ok::<_, RedisError>(())
  });



  sleep(Duration::from_millis(50000)).await;
  let _ = subscribe_task.abort();
  Ok(())
}