use std::time::Duration;

use yeelight::{Bulb, Effect, Mode, Power};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let my_bulb_ip = "192.168.1.200";
    let my_computer_ip = "192.168.1.23";

    let mut bulb = Bulb::connect(my_bulb_ip, 0).await?;
    let mut music_conn = bulb.start_music(my_computer_ip).await?;

    let sleep_duration = Duration::from_millis(300);
    let no_duration = Duration::from_millis(0);

    bulb.set_power(Power::On, Effect::Sudden, no_duration, Mode::Normal)
        .await?;

    for _ in 0..60 {
        std::thread::sleep(sleep_duration);
        music_conn
            .set_rgb(0x00ff00, Effect::Sudden, no_duration)
            .await?;
        std::thread::sleep(sleep_duration);
        music_conn
            .set_rgb(0x0000ff, Effect::Sudden, no_duration)
            .await?;
        std::thread::sleep(sleep_duration);
        music_conn
            .set_rgb(0xff0000, Effect::Sudden, no_duration)
            .await?;
    }

    drop(music_conn);

    Ok(())
}
