use mqtt::client::Client;

fn main() {
    let client = Client::new(String::from("test_client"), "0.0.0.0");
    client.connect();
    let topic = "testing/topic";
    let payload = "hello world";
    client.subscribe(topic);
    client.publish(topic, payload, 1, false);
    client.disconnect();
}
