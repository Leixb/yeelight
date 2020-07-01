use std::{thread, time};

use yeelight::*;

// This program is meant to demonstrate some examples of commands and how to read the results turns
// on the bulb, changes the brightness following the collatz sequence (mod 100) 10 times waiting 1
// second each, and then sets the color to red over 10 seconds.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bulb = Bulb::connect("192.168.1.204", 55443).await?;

    // Turn on the bulb
    println!(
        "Response: {:?}",
        bulb.set_power(Power::On, Effect::Sudden, 0, Mode::Normal).await?
    );

    // Define flow array
    let props = Properties(vec![
        Property::Power,
        Property::Bright,
        Property::CT,
        Property::RGB,
    ]);

    let second = time::Duration::from_millis(1000);

    for _ in 1..10u8 {
        let response = bulb.get_prop(&props).await?;
        let brightness = match response {
            Response::Result(result) => {
                println!("Properties: {:?}", result);
                result[1].parse::<u32>().unwrap()
            }
            Response::Error(code, message) => {
                eprintln!("Error (code {}): {}", code, message);
                std::process::exit(code);
            }
        };

        // Change brightness following collatz sequence
        let brightness = if brightness % 2 == 0 {
            brightness / 2
        } else {
            brightness * 3 + 1
        };

        // Make sure brightness is between 1 and 100.
        let brightness = (brightness % 100 + 1) as u8;
        println!("Setting brightness to {}", brightness);

        // Change brightness
        let response = bulb.set_bright(brightness, Effect::Smooth, 1000).await?;
        eprintln!("Response: {:?}", response);

        thread::sleep(second);
    }

    // Set bulb to pure red over 10 seconds
    let response = bulb.set_rgb(0xff_00_00, Effect::Smooth, 10000).await?;
    if let Response::Error(code, message) = response {
        eprintln!("Error (code {}): {}", code, message);
        std::process::exit(code);
    }
    Ok(())
}
