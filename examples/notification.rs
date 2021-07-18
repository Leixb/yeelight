use yeelight::Bulb;
use yeelight::Notification;

use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let bulb = Bulb::connect("192.168.1.204", 55443).await.unwrap();
    let mut bulb = bulb.no_response();

    let (sender, mut recv) = mpsc::channel(10);

    bulb.set_notify(sender).await;

    while let Some(Notification(i)) = recv.recv().await {
        for (k, v) in i.iter() {
            println!("{} {}", k, v);
        }
    }
}
