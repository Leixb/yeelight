use yeelight::*;

fn main() {

    let mut args = std::env::args();

    args.next(); // Drop program name
    
    let arg1 = args.next().expect("Please provide the IP as first argument");
    let mut location = arg1.split(':');

    let ip = location.next().expect("Invalid address");
    let port = location.next().unwrap_or("55443").parse::<u16>().expect("Invalid address");

    if location.next().is_some() {
        eprintln!("Invalid address");
        std::process::exit(1);
    }

    let bulb = Bulb::new(&ip, port);

    let method = args.next().expect("Please provide the method as second argument");
    let params = args.collect();

    match bulb.send_custom_message(&method, params) {
        Ok(message) => println!("{}", message),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        },
    }
}
