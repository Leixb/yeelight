use yeelight::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let my_bulb_ip = "192.168.1.204";
    let mut bulb = Bulb::connect(my_bulb_ip, 55443)?;

    // Turn on the bulb
    println!(
        "Response: {:?}",
        bulb.set_power(Power::On, Effect::Sudden, 0, Mode::Normal)
            .expect("Failed to communicate with bulb (set_power)")
    );

    // Define flow array
    let props = Properties::new(vec![
        Property::Power,
        Property::Bright,
        Property::CT,
        Property::RGB,
    ]);
    // Send flow command
    println!(
        "Response: {:?}",
        bulb.get_prop(props)
            .expect("Failed to communicate with bulb (get_prop)")
    );
    println!(
        "Response: {:?}",
        bulb.set_rgb(122, Effect::Smooth, 500)
            .expect("Failed to communicate with bulb (get_prop)")
    );
    Ok(())
}
