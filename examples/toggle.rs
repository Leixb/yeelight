extern crate yeelight;

use yeelight::Bulb;

fn main() {
    let my_bulb_ip = "192.168.1.204";
    let mut bulb = Bulb::new(my_bulb_ip, 55443).expect("Connection failed");
    bulb.toggle().expect("Failed to communicate with bulb");
}
