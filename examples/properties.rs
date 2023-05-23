use yeelight::{Bulb, Properties, Property};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let my_bulb_ip = "192.168.1.204";
    let mut bulb = Bulb::connect(my_bulb_ip, 55443).await?;

    // Define properties to query
    let props = Properties(vec![
        Property::Power,
        Property::Bright,
        Property::CT,
        Property::RGB,
    ]);
    // Print response with properties
    println!("Response: {:?}", bulb.get_prop(&props).await?);
    Ok(())
}
