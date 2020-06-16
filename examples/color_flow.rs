extern crate yeelight;

use yeelight::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_bulb_ip = "192.168.1.204";
    let mut bulb = Bulb::connect(my_bulb_ip, 55443)?;

    // Turn on the bulb
    let response = bulb
        .set_power(Power::On, Effect::Sudden, 0, Mode::Normal)
        .expect("Failed to communicate with bulb (set_power)");
    println!("response: {:?}", response);

    // Define flow array
    let flow = FlowExpresion {
        expr: vec![
            FlowTuple::ct(500, 3000, 100),
            FlowTuple::sleep(1500),
            FlowTuple::ct(500, 5000, 100),
            FlowTuple::sleep(1500),
            FlowTuple::ct(500, 2600, 100),
            FlowTuple::sleep(1500),
        ],
    };
    // Send flow command
    let response = bulb
        .start_cf(10, CfAction::Stay, flow)
        .expect("Failed to communicate with bulb (start_cf)");
    println!("response: {:?}", response);
    Ok(())
}
