use yeelight::Bulb;

#[tokio::main]
async fn main() {

    env_logger::init();

    let my_bulb_ip = "192.168.1.200";
    let mut bulb = Bulb::connect(my_bulb_ip, 55443)
        .await
        .expect("Connection failed");
    if let Some(response) = bulb.toggle().await.expect("Error") {
        for v in response.iter() {
            println!("{}", v);
        }
    }
}
