extern crate yeelight;

use yeelight::*;

fn main() {
    let my_bulb_ip = "192.168.1.204";
    let bulb = Bulb::new(my_bulb_ip, 55443);

    // Turn on the bulb
    bulb.set_power(Power::On, Effect::Sudden, 0, Mode::Normal).expect("Failed to communicate with bulb (set_power)");

    // Define flow array
    let flow = &[
        &FlowTuple::ct(500, 3000, 100),
        &FlowTuple::sleep(1500),
        &FlowTuple::ct(500, 5000, 100),
        &FlowTuple::sleep(1500),
        &FlowTuple::ct(500, 2600, 100),
        &FlowTuple::sleep(1500),
    ];
    // Send flow command
    bulb.start_cf(10, CfAction::Stay, flow).expect("Failed to communicate with bulb (start_cf)");
}
