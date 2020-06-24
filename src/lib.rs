mod reader;
mod writer;

use reader::Reader;
pub use reader::Response;
use writer::Writer;

use tokio::net::TcpStream;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::mpsc;

use tokio::task::spawn;

use std::error::Error;

use serde::{Deserialize, Serialize};

#[cfg(feature = "from-str")]
use itertools::Itertools;

#[derive(Debug)]
struct Message(u64, String);

use tokio::net::tcp::OwnedReadHalf;

pub use reader::Notification;
use reader::NotifyChan;

/// Bulb connection
pub struct Bulb {
    notify_chan: NotifyChan,
    writer: writer::Writer,
}

impl Bulb {
    /*
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
    */
    pub fn attach(stream: std::net::TcpStream) -> Result<Bulb, Box<dyn Error>> {
        let stream = TcpStream::from_std(stream)?;

        let (reader, writer, reader_half, notify_chan) = Self::build_rw(stream);

        spawn(reader.start(reader_half));

        Ok(Bulb {
            notify_chan,
            writer,
        })
    }
    /*

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
    */
    pub async fn connect(addr: &str, port: u16) -> Result<Bulb, Box<dyn Error>> {

        let stream = TcpStream::connect(format!("{}:{}", addr, port)).await?;

        let (reader, writer, reader_half, notify_chan) = Self::build_rw(stream);

        spawn(reader.start(reader_half));

        Ok(Bulb {
            notify_chan,
            writer,
        })
    }

    fn build_rw(stream: TcpStream) -> (Reader, Writer, OwnedReadHalf, NotifyChan) {
        let (reader_half, writer_half) = stream.into_split();
        
        let resp_chan = HashMap::new();
        let resp_chan = Arc::new(Mutex::new(resp_chan));
        let notify_chan = Arc::new(Mutex::new(None));

        let reader = Reader::new(resp_chan.clone(), notify_chan.clone());

        let writer = Writer::new(writer_half, resp_chan);

        (reader, writer, reader_half, notify_chan)
    }

    pub fn no_response(mut self) -> Self {
        self.writer.set_get_response(false);
        self
    }

    pub fn get_response(mut self) -> Self {
        self.writer.set_get_response(true);
        self
    }

    pub async fn set_notify(&mut self, chan: mpsc::Sender<Notification>) -> () {
        self.notify_chan.lock().await.replace(chan);
    }
}

#[cfg(feature = "from-str")]
#[derive(Debug)]
pub struct ParseError(String);

#[cfg(feature = "from-str")]
impl ToString for ParseError {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

#[cfg(feature = "from-str")]
impl From<std::num::ParseIntError> for ParseError {
    fn from(e: std::num::ParseIntError) -> ParseError {
        ParseError(e.to_string())
    }
}

// Create enum and its ToString implementation using stringify (quoted strings)
macro_rules! enum_str {
    ($name:ident: $($variant:ident -> $val:literal),* $(,)?) => {

        #[derive(Debug, Clone, Copy, Serialize, Deserialize)]
        pub enum $name {
            $($variant),*
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match *self {
                    $($name::$variant => write!(f, stringify!($val)),)+
                }
            }
        }

        #[cfg(feature="from-str")]
        impl ::std::str::FromStr for $name {
            type Err = ParseError;

            fn from_str(s: &str) -> Result<Self,Self::Err> {
                match s {
                    $(stringify!($variant) |
                    _ if s.eq_ignore_ascii_case(stringify!($variant)) => Ok($name::$variant),)+
                    _                => Err(ParseError({
                                            let v = vec![
                                                $(stringify!($variant),)+
                                            ];
                                            format!("Could not parse {} \n Valid values:{}", s,
                                                v.iter().fold(String::new(), |a, i| {
                                                    a + &format!(" {}", i)[..]
                                                }))
                                        }))
                }
            }
        }

        #[cfg(feature="from-str")]
        impl $name {
            #[allow(dead_code)]
            pub fn variants() -> Vec<&'static str> {
                vec![
                    $(stringify!($variant),)+
                ]
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
///
#[derive(Debug, Serialize, Deserialize)]
pub struct FlowTuple {
    pub duration: u64,
    pub mode: FlowMode,
    pub value: u32,
    pub brightness: i8,
}

impl FlowTuple {
    /// Create FlowTuple specifying the mode as a parameter
    /// # Arguments
    ///
    /// * `duration`: duration of change in milliseconds.
    /// * `mode`: [`FlowMode`](enum.FlowMode.html) Color / CT / Sleep.
    /// * `value`: RGB color for color mode, CT for ct mode (ignored by sleep)
    /// * `brightness`: percentage (`1` to `100`) `-1` to keep previous value (ignored by sleep)
    ///
    pub fn new(duration: u64, mode: FlowMode, value: u32, brightness: i8) -> FlowTuple {
        FlowTuple {
            duration,
            mode,
            value,
            brightness,
        }
    }

    /// Create RGB FlowTuple
    ///
    /// # Arguments
    ///
    /// * `duration`: duration of change in milliseconds.
    /// * `rgb`: color in RGB format (`0x00_00_00` to `0xff_ff_ff`)
    /// * `brightness`: percentage (`1` to `100`) `-1` to keep previous value.
    ///
    pub fn rgb(duration: u64, rgb: u32, brightness: i8) -> FlowTuple {
        FlowTuple {
            duration,
            mode: FlowMode::Color,
            value: rgb,
            brightness,
        }
    }

    /// Create Color Temperature FlowTuple
    ///
    /// # Arguments
    ///
    /// * `duration`: duration of change in milliseconds.
    /// * `ct`: color temperature (`1600` to `6000`) K (may vary between models).
    /// * `brightness`: percentage (`1` to `100`) `-1` to keep previous value.
    ///
    pub fn ct(duration: u64, ct: u32, brightness: i8) -> FlowTuple {
        FlowTuple {
            duration,
            mode: FlowMode::CT,
            value: ct,
            brightness,
        }
    }

    /// Create Sleep FlowTuple
    ///
    /// # Arguments
    ///
    /// * `duration`: time to sleep in milliseconds
    ///
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
///
/// # Example
///```
///# use yeelight::{FlowTuple, FlowExpresion};
/// let duration = 1000; // miliseconds
/// let brightness = 100; // percentage 1..100 (-1 to keep previous)
///
/// let police = FlowExpresion(vec![
///     FlowTuple::rgb(duration, 0xff_00_00, brightness),
///     FlowTuple::rgb(duration, 0x00_00_ff, brightness),
/// ]);
///
/// let police2 = FlowExpresion(vec![
///     FlowTuple::rgb(duration, 0xff_00_00, brightness),
///     FlowTuple::rgb(duration, 0xff_00_00, 1),
///     FlowTuple::rgb(duration, 0xff_00_00, brightness),
///     FlowTuple::sleep(duration),
///     FlowTuple::rgb(duration, 0x00_00_ff, brightness),
///     FlowTuple::rgb(duration, 0x00_00_ff, 1),
///     FlowTuple::rgb(duration, 0x00_00_ff, brightness),
///     FlowTuple::sleep(duration),
/// ]);
///```
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

#[cfg(feature = "from-str")]
impl std::str::FromStr for FlowExpresion {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut v = Vec::new();
        for (duration, mode, value, brightness) in s.split(',').tuples() {
            let duration = duration.parse::<u64>()?;
            let value = value.parse::<u32>()?;
            let mode = match FlowMode::from_str(mode) {
                Ok(m) => Ok(m),
                Err(_) => match mode {
                    "1" => Ok(FlowMode::Color),
                    "2" => Ok(FlowMode::CT),
                    "7" => Ok(FlowMode::Sleep),
                    _ => Err(ParseError(format!(
                        "Could not parse FlowMode: {}\nvalid values: 1 (Color), 2(CT), 7(Sleep)",
                        mode.to_string()
                    ))),
                },
            }?;
            let brightness = brightness.parse::<i8>()?;
            v.push(FlowTuple {
                duration,
                mode,
                value,
                brightness,
            });
        }
        Ok(FlowExpresion(v))
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

            pub async fn $name(&mut self, $($p : $t),*) -> Option<Response> {
                self.writer.send(
                    &stringify!($name), &params!($($p),*)
                ).await
            }

    };
    ($fn_default:ident / $fn_bg:ident - $( $p:ident : $t:ty ),* ) => {

        gen_func!($fn_default - $($p : $t),*);
        gen_func!($fn_bg - $($p : $t),*);

    };
    ($name:ident) => { gen_func!($name - ); };
    ($fn_default:ident / $fn_bg:ident) => { gen_func!($fn_default / $fn_bg - ); };
}

pub struct QuotedString(pub String);

impl ToString for QuotedString {
    fn to_string(&self) -> String {
        let mut s = String::with_capacity(self.0.len() + 2);
        s.push('"');
        s.push_str(&self.0);
        s.push('"');
        s
    }
}

/// # Messages
///
/// This are all the methods as by the yeelight API spec.
/// They all take the parameters specified by the spec, build a message, send
/// it to the bulb and wait for the response which is parsed into a
/// [`Response`].
///
/// [`Response`]: enum.Response.html
#[rustfmt::skip]
impl Bulb {
    gen_func!(get_prop                          - properties: &Properties);

    gen_func!(set_power     / bg_set_power      - power: Power,         effect: Effect, duration: u64, mode: Mode);
    gen_func!(toggle        / bg_toggle);
    gen_func!(dev_toggle);

    gen_func!(set_ct_abx    / bg_set_ct_abx     - ct_value: u64,        effect: Effect, duration: u64);
    gen_func!(set_rgb       / bg_set_rgb        - rgb_value: u32,       effect: Effect, duration: u64);
    gen_func!(set_hsv       / bg_set_hsv        - hue: u16, sat: u8,    effect: Effect, duration: u64);
    gen_func!(set_bright    / bg_set_bright     - brightness: u8,       effect: Effect, duration: u64);
    gen_func!(set_scene     / bg_set_scene      - class: Class,         val1: u64, val2: u64, val3: u64);

    gen_func!(start_cf      / bg_start_cf       - count: u8, action: CfAction, flow_expression: FlowExpresion);
    gen_func!(stop_cf       / bg_stop_cf);

    gen_func!(set_adjust    / bg_set_adjust     - action: AdjustAction, prop: Prop);
    gen_func!(adjust_bright / bg_adjust_bright  - percentage: i8, duration: u64);
    gen_func!(adjust_ct     / bg_adjust_ct      - percentage: i8, duration: u64);
    gen_func!(adjust_color  / bg_adjust_color   - percentage: i8, duration: u64);

    gen_func!(set_default   / bg_set_default);

    gen_func!(set_name                          - name: QuotedString);
    gen_func!(set_music                         - action: MusicAction, host: QuotedString, port: u32);

    gen_func!(cron_add                          - cron_type: CronType, value: u64);
    gen_func!(cron_del                          - cron_type: CronType);
    // gen_func!(cron_get                          - cron_type: CronType);
    // cron_get response is a dictionary which is difficult to parse,
    // instead use delayoff property which should give the same values.
    pub async fn cron_get(&mut self, _cron_type: CronType) -> Option<Response> {
        self.get_prop(&Properties(vec![Property::DelayOff])).await
    }
}
