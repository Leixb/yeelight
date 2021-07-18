use std::time::Duration;

use yeelight::{Bulb, CfAction, Effect, FlowExpresion, FlowTuple, Mode, Power};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_bulb_ip = "192.168.1.204";
    let mut bulb = Bulb::connect(my_bulb_ip, 55443).await?;

    // Turn on the bulb
    let response = bulb
        .set_power(Power::On, Effect::Sudden, Duration::new(0, 0), Mode::Normal)
        .await?;
    println!("response: {:?}", response);

    // Define flow array
    let flow = FlowExpresion(vec![
        FlowTuple::ct(Duration::from_millis(500), 3000, 100),
        FlowTuple::sleep(Duration::from_millis(1500)),
        FlowTuple::ct(Duration::from_millis(500), 5000, 100),
        FlowTuple::sleep(Duration::from_millis(1500)),
        FlowTuple::ct(Duration::from_millis(500), 2600, 100),
        FlowTuple::sleep(Duration::from_millis(1500)),
    ]);
    // Send flow command
    let response = bulb.start_cf(10, CfAction::Stay, flow).await?;
    println!("response: {:?}", response);
    Ok(())
}
