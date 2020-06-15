use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

pub struct Bulb {
    stream: TcpStream,
    message_id: u64,
}

macro_rules! enum_str {
    ($name:ident: $($variant:ident -> $val:literal),* $(,)?) => {
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

pub struct FlowTuple {
    pub duration: u64,
    pub mode: FlowMode,
    pub value: u32,
    pub brightness: i8,
}

impl FlowTuple {
    pub fn new(duration: u64, mode: FlowMode, value: u32, brightness: i8) -> FlowTuple {
        FlowTuple {
            duration,
            mode,
            value,
            brightness,
        }
    }

    pub fn rgb(duration: u64, rgb: u32, brightness: i8) -> FlowTuple {
        FlowTuple {
            duration,
            mode: FlowMode::Color,
            value: rgb,
            brightness,
        }
    }
    pub fn ct(duration: u64, ct: u32, brightness: i8) -> FlowTuple {
        FlowTuple {
            duration,
            mode: FlowMode::CT,
            value: ct,
            brightness,
        }
    }
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

pub struct FlowExpresion {
    pub expr: Vec<FlowTuple>,
}

impl FlowExpresion {
    pub fn new(expr: Vec<FlowTuple>) -> FlowExpresion {
        FlowExpresion { expr }
    }
}

impl ToString for FlowExpresion {
    fn to_string(&self) -> String {
        let mut s = '"'.to_string();
        for tuple in self.expr.iter() {
            s.push_str(&tuple.to_string());
            s.push(',');
        }
        s.pop();
        s.push('"');
        s
    }
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

pub struct Properties {
    pub properties: Vec<Property>,
}

impl Properties {
    pub fn new(properties: Vec<Property>) -> Properties {
        Properties { properties }
    }
}

impl ToString for Properties {
    fn to_string(&self) -> String {
        self.properties
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

macro_rules! params {
    ($($v:tt),+) => {
        vec!( $( $v.to_string() ),+ ).join(", ")
    };
    () => {""};
}

macro_rules! gen_func {
    ($name:ident - $( $p:ident : $t:ty ),* ) => {
        impl Bulb {

            pub fn $name(&mut self, $($p : $t),*) -> Result<String, std::io::Error> {
                let message = self.craft_message(&stringify!($name), &params!($($p),*));
                self.send(message)
            }

        }
    };
    ($name:ident) => { gen_func!($name - ); };
}

gen_func!(get_prop - properties: Properties);
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

struct Message(u64, String);

impl Bulb {
    pub fn new(ip: &str, port: u16) -> std::result::Result<Bulb, std::io::Error> {
        Ok(Bulb {
            stream: TcpStream::connect(format!("{}:{}", ip, port))?,
            message_id: 0,
        })
    }

    fn send(&mut self, message: Message) -> std::result::Result<String, std::io::Error> {

        self.stream.write_all(message.1.as_bytes())?;

        let reader = BufReader::new(&self.stream);

        let mut line = String::new();

        let start = format!(r#"{{"id":{},"#, message.0);

        let mut lines_iter = reader.lines();
        while !line.starts_with(&start) {
            match lines_iter.next() {
                Some(l) => line = l?,
                None => break,
            }
        }

        Ok(line)
    }

    fn get_message_id(&mut self) -> u64 {
        self.message_id += 1;
        self.message_id
    }

    fn craft_message(&mut self, method: &str, params: &str) -> Message {
        let id = self.get_message_id();
        Message(id, format!(
            r#"{{ "id": {}, "method": "{}", "params": [{} ] }}"#,
            id, method, params
        ) + "\r\n")
    }
}
