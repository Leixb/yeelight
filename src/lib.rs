//! This is crate provides rust Bindings for yeelight API.
//!
//! It implements methods as listed in [Yeelight_Inter-Operation_spec.pdf][1].
//! To communicate with the bulbs you have to enable LAN mode in the yeelight app.
//!
//! Currently the TCP connection to the bulb is maintained for the whole life
//! of the `Bulb` object. This could cause a problem since the light bulbs
//! only support 4 simultaneous TCP connections.
//!
//! This is a work in progress, the following features are still in the works:
//!
//! - Allow creation of bulb that creates and closes connections for each message.
//! - Handle messages and responses asynchronously.
//! - Handle notification messages.
//! - Discover bulbs in LAN.
//! - Support for music mode.
//!
//! # Example
//!
//! This example shows some methods and how to parse the responses. It turns on the bulb and
//! changes the brightness following the collatz sequence (applied 10 times), waiting 1 second for
//! each iteration. More examples are provided in the examples folder.
//!
//! ```
//! use std::{thread, time};
//!
//! use yeelight::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut bulb = Bulb::connect("192.168.1.204", 55443)?;
//!
//!     // Turn on the bulb
//!     bulb.set_power(Power::On, Effect::Sudden, 0, Mode::Normal)?;
//!
//!     let second = time::Duration::from_millis(1000);
//!
//!     // Define vector with properties to query
//!     let props = Properties(vec![ Property::Bright ]);
//!     for  _ in 1..10 {
//!         let response = bulb.get_prop(&props)?;
//!         let brightness = match response {
//!             Response::Error{error: ErrDetails{code, message}, ..} => {
//!                 eprintln!("Error (code {}): {}", code, message);
//!                 std::process::exit(code);
//!             }
//!             Response::Result{result,..} => result[0].parse::<u32>().unwrap(),
//!             Response::Notification{..} => panic!("This should not happen"),
//!         };
//!
//!         // Change brightness following collatz sequence
//!         let brightness = if brightness%2 == 0 {
//!             brightness/2
//!         } else {
//!             brightness*3 + 1
//!         };
//!
//!         // Make sure brightness is between 1 and 100.
//!         let brightness = (brightness%100 + 1) as u8;
//!         println!("Setting brightness to {}", brightness);
//!
//!         // Change brightness smooth over 1 second
//!         let response = bulb.set_bright(brightness, Effect::Smooth, 1000)?;
//!         eprintln!("Response: {:?}", response);
//!
//!         // Sleep for 1 second
//!         thread::sleep(second);
//!     }
//!     Ok(())
//! }
//! ```
//!
//!  [1]: https://www.yeelight.com/download/Yeelight_Inter-Operation_Spec.pdf
//!

use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

use serde::{Deserialize, Serialize};

type ResultResponse = std::result::Result<Response, std::io::Error>;

#[derive(Debug)]
struct Message(u64, String);

/// Parsed response from the bulb.
///
/// Can be either Result, Error or a Notification indicating
/// a state change.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Result {
        id: u64,
        result: Vec<String>,
    },
    Error {
        id: u64,
        error: ErrDetails,
    },
    Notification {
        method: String,
        params: serde_json::Map<String, serde_json::Value>,
    },
}

/// Error details (from `Response::Error`)
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrDetails {
    pub code: i32,
    pub message: String,
}

/// Bulb connection
pub struct Bulb {
    stream: TcpStream,
    message_id: u64,
}

impl Bulb {
    /// Attach to existing `TcpStream`.
    ///
    /// # Example
    /// ```
    /// # use yeelight::Bulb;
    /// let stream = std::net::TcpStream::connect("192.168.1.204:55443")
    ///     .expect("Connection failed");
    /// let mut bulb = Bulb::attach(stream);
    /// bulb.toggle().unwrap();
    /// ```
    pub fn attach(stream: TcpStream) -> Bulb {
        Bulb {
            stream,
            message_id: 0,
        }
    }

    /// Connect to bulb at the specified address and port.
    ///
    /// # Example
    /// ```
    /// # use yeelight::Bulb;
    /// let my_bulb_ip = "192.168.1.204";
    /// let mut bulb = Bulb::connect(my_bulb_ip, 55443)
    ///     .expect("Connection failed");
    /// bulb.toggle().unwrap();
    /// ```
    pub fn connect(addr: &str, port: u16) -> std::result::Result<Bulb, std::io::Error> {
        Ok(Bulb {
            stream: TcpStream::connect(format!("{}:{}", addr, port))?,
            message_id: 0,
        })
    }

    fn send(&mut self, message: Message) -> ResultResponse {
        let Message(id, content) = message;

        self.stream.write_all(content.as_bytes())?;

        let reader = BufReader::new(&self.stream);

        for line in reader.lines() {
            let r: Response = serde_json::from_slice(&line?.into_bytes())?;
            match &r {
                Response::Result { id: resp_id, .. } => {
                    if *resp_id == id {
                        return Ok(r);
                    }
                }
                Response::Error { id: resp_id, .. } => {
                    if *resp_id == id {
                        return Ok(r);
                    }
                }
                _ => (),
            }
        }
        Ok(Response::Error {
            id,
            error: ErrDetails {
                code: -1,
                message: "No response".to_string(),
            },
        })
    }

    fn get_message_id(&mut self) -> u64 {
        self.message_id += 1;
        self.message_id
    }

    fn craft_message(&mut self, method: &str, params: &str) -> Message {
        let id = self.get_message_id();
        Message(
            id,
            format!(
                r#"{{ "id": {}, "method": "{}", "params": [{} ] }}"#,
                id, method, params
            ) + "\r\n",
        )
    }
}

// Create enum and its ToString implementation using stringify (quoted strings)
macro_rules! enum_str {
    ($name:ident: $($variant:ident -> $val:literal),* $(,)?) => {
        #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
        pub enum $name {
            $($variant),*
        }

        impl ToString for $name {
            fn to_string(&self) -> String {
                match self {
                    $($name::$variant => stringify!($val)),*
                }.to_string()
            }
        }
    };
}

enum_str!(Property:
    Power -> "power",
    Bright -> "bright",
    CT -> "ct",
    RGB -> "rgb",
    Hue -> "hue",
    Sat -> "sat",
    ColorMode -> "color_mode",
    Flowing -> "flowing",
    DelayOff -> "delayoff",
    FlowParams -> "flow_params",
    MusicOn -> "music_on",
    Name -> "name",
    BgPower -> "bg_power",
    BgFlowing -> "bg_flowing",
    BgFlowParams -> "bg_flow_params",
    BgCT -> "bg_ct",
    BgColorMode -> "bg_lmode",
    BgBright -> "bg_bright",
    BgRGB -> "bg_rgb",
    BgHue -> "bg_hue",
    BgSat -> "bg_sat",
    NightLightBright -> "nl_br",
    ActiveMode -> "active_mode",
);

enum_str!(Power:
    On -> "on",
    Off -> "off",
);
enum_str!(Effect:
    Sudden -> "sudden",
    Smooth -> "smooth",
);
enum_str!(Prop:
    Bright -> "bright",
    CT -> "ct",
    Color -> "color",
);
enum_str!(Class:
    Color -> "color",
    HSV -> "hsv",
    CT -> "ct",
    CF -> "cf",
    AutoDelayOff -> "auto_delay_off",
);
enum_str!(Mode:
    Normal -> 0,
    CT -> 1,
    RGB -> 2,
    HSV -> 3,
    CF -> 4,
    NightLight -> 5,
);
enum_str!(CronType:
    Off -> 0,
);
enum_str!(CfAction:
    Recover -> 0,
    Stay -> 1,
    Off -> 2,
);
enum_str!(AdjustAction:
    Increase -> "increase",
    Decrease -> "decrease",
    Circle -> "circle",
);
enum_str!(MusicAction:
    Off -> 0,
    On -> 1,
);
enum_str!(FlowMode:
    Color -> 1,
    CT -> 2,
    Sleep -> 7,
);

/// State Change used to build [`FlowExpresion`](struct.FlowExpresion.html)s
///
/// The state change can be either: color (rgb), color temperature (ct) or sleep.
#[derive(Debug, Serialize, Deserialize)]
pub struct FlowTuple {
    pub duration: u64,
    pub mode: FlowMode,
    pub value: u32,
    pub brightness: i8,
}

impl FlowTuple {
    /// Create FlowTuple specifying the mode as a parameter
    pub fn new(duration: u64, mode: FlowMode, value: u32, brightness: i8) -> FlowTuple {
        FlowTuple {
            duration,
            mode,
            value,
            brightness,
        }
    }

    /// Create RGB FlowTuple
    pub fn rgb(duration: u64, rgb: u32, brightness: i8) -> FlowTuple {
        FlowTuple {
            duration,
            mode: FlowMode::Color,
            value: rgb,
            brightness,
        }
    }
    /// Create Color Temperature FlowTuple
    pub fn ct(duration: u64, ct: u32, brightness: i8) -> FlowTuple {
        FlowTuple {
            duration,
            mode: FlowMode::CT,
            value: ct,
            brightness,
        }
    }
    /// Create Sleep FlowTuple
    pub fn sleep(duration: u64) -> FlowTuple {
        FlowTuple {
            duration,
            mode: FlowMode::Sleep,
            value: 0,
            brightness: -1,
        }
    }
}

impl ToString for FlowTuple {
    fn to_string(&self) -> String {
        format!(
            "{},{},{},{}",
            self.duration,
            self.mode.to_string(),
            self.value,
            self.brightness
        )
    }
}

/// FlowExpresion consisting of a series of `FlowTuple`s
#[derive(Debug, Serialize, Deserialize)]
pub struct FlowExpresion(pub Vec<FlowTuple>);

impl ToString for FlowExpresion {
    fn to_string(&self) -> String {
        let mut s = '"'.to_string();
        for tuple in self.0.iter() {
            s.push_str(&tuple.to_string());
            s.push(',');
        }
        s.pop();
        s.push('"');
        s
    }
}


/// List of `Property` (used by `get_prop`)
///
/// # Example
///```
///# use yeelight::{Properties, Property};
/// let props = Properties(vec![
///     Property::Name,
///     Property::Power,
///     Property::Bright,
///     Property::CT,
///     Property::RGB,
///     Property::ColorMode,
///     Property::Flowing,
/// ]);
///```
#[derive(Debug, Serialize, Deserialize)]
pub struct Properties(pub Vec<Property>);

impl ToString for Properties {
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

// Convert function parameters into comma separated string
macro_rules! params {
    ($($v:tt),+) => {
        vec!( $( $v.to_string() ),+ ).join(", ")
    };
    () => {""};
}

// Generate function
macro_rules! gen_func {
    ($name:ident - $( $p:ident : $t:ty ),* ) => {

            pub fn $name(&mut self, $($p : $t),*) -> ResultResponse {
                let message = self.craft_message(&stringify!($name), &params!($($p),*));
                self.send(message)
            }

    };
    ($name:ident) => { gen_func!($name - ); };
}

/// # Messages
///
/// This are all the methods as by the yeelight API spec.
/// They all take the parameters specified by the spec, build a message, send
/// it to the bulb and wait for the response which is parsed into a
/// [`Response`].
///
/// [`Response`]: enum.Response.html
impl Bulb {
    gen_func!(get_prop - properties: &Properties);
    gen_func!(set_ct_abx - ct_value: u64, effect: Effect, duration: u64);
    gen_func!(set_rgb - rgb_value: u32, effect: Effect, duration: u64);
    gen_func!(set_hsv - hue: u16, sat: u8, effect: Effect, duration: u64);
    gen_func!(set_bright - brightness: u8, effect: Effect, duration: u64);
    #[rustfmt::skip]
    gen_func!(set_power - power: Power, effect: Effect, duration: u64, mode: Mode);
    gen_func!(toggle);
    gen_func!(set_default);
    #[rustfmt::skip]
    gen_func!(start_cf - count: u8, action: CfAction, flow_expression: FlowExpresion);
    gen_func!(stop_cf);

    gen_func!(set_scene - class: Class, val1: u64, val2: u64, val3: u64);

    gen_func!(cron_add - cron_type: CronType, value: u64);
    gen_func!(cron_get - cron_type: CronType);
    gen_func!(cron_del - cron_type: CronType);

    gen_func!(set_adjust - action: AdjustAction, prop: Prop);
    gen_func!(set_music - action: MusicAction, host: &str, port: u32);

    gen_func!(set_name - name: &str);

    gen_func!(bg_set_rgb - rgb_value: u32, effect: Effect, duration: u64);
    #[rustfmt::skip]
    gen_func!(bg_set_hsv - hue: u16, sat: u8, effect: Effect, duration: u64);
    gen_func!(bg_set_ct_abx - ct_value: u64, effect: Effect, duration: u64);

    #[rustfmt::skip]
    gen_func!(bg_start_cf - count: u8, action: CfAction, flow_expression: FlowExpresion);
    gen_func!(bg_stop_cf);

    gen_func!(bg_set_scene - class: Class, val1: u64, val2: u64, val3: u64);
    gen_func!(bg_set_default);

    #[rustfmt::skip]
    gen_func!(bg_set_power - power: Power, effect: Effect, duration: u64, mode: Mode);
    #[rustfmt::skip]
    gen_func!(bg_set_bright - brightness: u8, effect: Effect, duration: u64);
    gen_func!(bg_set_adjust - action: AdjustAction, prop: Prop);
    gen_func!(bg_toggle);

    gen_func!(dev_toggle);

    gen_func!(adjust_bright - percentage: u8, duration: u64);
    gen_func!(adjust_ct - percentage: u8, duration: u64);
    gen_func!(adjust_color - percentage: u8, duration: u64);

    gen_func!(bg_adjust_bright - percentage: u8, duration: u64);
    gen_func!(bg_adjust_ct - percentage: u8, duration: u64);
    gen_func!(bg_adjust_color - percentage: u8, duration: u64);
}
