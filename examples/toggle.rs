use yeelight::Bulb;

#[tokio::main]
async fn main() {
    let my_bulb_ip = "192.168.1.204";
    let mut bulb = Bulb::connect(my_bulb_ip, 55443)
        .await
        .expect("Connection failed");
    if let Some(response) = bulb.toggle().await.expect("Error") {
        for v in response.iter() {
            println!("{}", v);
        }
    }
}
