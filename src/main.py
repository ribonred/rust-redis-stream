#[macro_use]
extern crate serde_json;
extern crate redis;
use serde_json::Value;




fn connect() -> redis::Connection {
    let client = redis::Client::open("redis://127.0.0.1/").expect("Failed to connect to Redis URL");
    client.get_connection().unwrap()
}
fn setter(key: &str, value: &str) {
    let mut conn: redis::Connection = connect();
    let _: () = redis::cmd("SET")
        .arg(key)
        .arg(value)
        .query(&mut conn)
        .expect("SET Error");
}
fn setter_json(key: &str, value: Value) {
    setter(key, value.to_string().as_str())
}
fn getter(key: &str) -> Option<String> {
    let mut conn: redis::Connection = connect();
    let value: Option<String> = redis::cmd("GET")
        .arg(key)
        .query(&mut conn)
        .expect("GET Error");
    value
}
fn getter_json(key: &str) -> Value {
    let val = getter(key);
    match val{
        Some(v) => v.parse().unwrap(),
        None => "{}".parse().unwrap(),
    }
}
fn listen(channel:&str){
    let  mut conn: redis::Connection = connect();
    let mut subsriber = conn.as_pubsub();
    subsriber.subscribe(channel).expect("subscribe error");
    loop{

        let msg = subsriber.get_message().expect("get message error");
        let payload :String= msg.get_payload().expect("get payload error");
        // let payload_str = str::from_utf8(payload).unwrap();
        print!("{},{}",msg.get_channel_name(), payload);
    }


}

    
    



fn main() {
    listen("market");

}
