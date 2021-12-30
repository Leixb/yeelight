use std::time::Duration;

use yeelight::{Bulb, Effect, Mode, Power, Properties, Property};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::init();

    let my_bulb_ip = "192.168.1.204";
    let mut bulb = Bulb::connect(my_bulb_ip, 55443).await?;

    // Turn on the bulb
    println!(
        "Response: {:?}",
        bulb.set_power(
            Power::On,
            Effect::Sudden,
            Duration::from_millis(0),
            Mode::Normal
        )
        .await?
    );

    // Define flow array
    let props = Properties(vec![
        Property::Power,
        Property::Bright,
        Property::CT,
        Property::RGB,
    ]);
    // Send flow command
    println!("Response: {:?}", bulb.get_prop(&props).await?);
    println!(
        "Response: {:?}",
        bulb.set_rgb(122, Effect::Smooth, Duration::from_millis(500))
            .await?
    );
    Ok(())
}
