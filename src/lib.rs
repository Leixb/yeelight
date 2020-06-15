use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

pub struct Bulb {
    ip: String,
    port: u16,
}

pub enum Power {
    On,
    Off,
}
pub enum Effect {
    Sudden,
    Smooth,
}
pub enum Prop {
    Bright,
    CT,
    Color,
}
pub enum Class {
    Color,
    HSV,
    CT,
    CF,
    AutoDelayOff,
}
pub enum Mode {
    Normal,
    CT,
    RGB,
    HSV,
    CF,
    NightLight,
}
pub enum CronType {
    Off,
}

pub enum CfAction {
    Recover,
    Stay,
    Off,
}
pub enum AdjustAction {
    Increase,
    Decrease,
    Circle,
}
pub enum MusicAction {
    On,
    Off,
}

pub enum FlowMode {
    Color,
    CT,
    Sleep,
}

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

trait QuoteStrings {
    fn quote(&self) -> String;
}

macro_rules! gen_quote_int {
    ($type:ty) => {
        impl QuoteStrings for $type {
            fn quote(&self) -> String {
                self.to_string()
            }
        }
    };
}

gen_quote_int!(u64);
gen_quote_int!(u32);
gen_quote_int!(u16);
gen_quote_int!(u8);

impl QuoteStrings for str {
    fn quote(&self) -> String {
        format!(r#""{}""#, self)
    }
}

impl QuoteStrings for Power {
    fn quote(&self) -> String {
        match self {
            Power::On => "on",
            Power::Off => "off",
        }
        .quote()
    }
}

impl QuoteStrings for Effect {
    fn quote(&self) -> String {
        match self {
            Effect::Sudden => "sudden",
            Effect::Smooth => "smooth",
        }
        .quote()
    }
}

impl QuoteStrings for AdjustAction {
    fn quote(&self) -> String {
        match self {
            AdjustAction::Increase => "increase",
            AdjustAction::Decrease => "decrease",
            AdjustAction::Circle => "circle",
        }
        .quote()
    }
}

impl QuoteStrings for CfAction {
    fn quote(&self) -> String {
        match self {
            CfAction::Recover => 0,
            CfAction::Stay => 1,
            CfAction::Off => 2,
        }
        .to_string()
    }
}

impl QuoteStrings for MusicAction {
    fn quote(&self) -> String {
        match self {
            MusicAction::Off => 0,
            MusicAction::On => 1,
        }
        .to_string()
    }
}

impl QuoteStrings for Prop {
    fn quote(&self) -> String {
        match self {
            Prop::Bright => "bright",
            Prop::CT => "ct",
            Prop::Color => "color",
        }
        .quote()
    }
}

impl QuoteStrings for Class {
    fn quote(&self) -> String {
        match self {
            Class::Color => "color",
            Class::HSV => "hsv",
            Class::CT => "ct",
            Class::CF => "cf",
            Class::AutoDelayOff => "auto_delay_off",
        }
        .quote()
    }
}

impl QuoteStrings for Mode {
    fn quote(&self) -> String {
        match self {
            Mode::Normal => 0,
            Mode::CT => 1,
            Mode::RGB => 2,
            Mode::HSV => 3,
            Mode::CF => 4,
            Mode::NightLight => 5,
        }
        .to_string()
    }
}

impl QuoteStrings for CronType {
    fn quote(&self) -> String {
        0.to_string()
    }
}

impl QuoteStrings for &[&str] {
    fn quote(&self) -> String {
        let v: Vec<String> = self.iter().map(|x| x.quote()).collect();
        v.join(", ")
    }
}

impl QuoteStrings for FlowTuple {
    fn quote(&self) -> String {
        format!(
            "{},{},{},{}",
            self.duration,
            self.mode.quote(),
            self.value,
            self.brightness
        )
    }
}

impl QuoteStrings for FlowMode {
    fn quote(&self) -> String {
        match self {
            FlowMode::Color => 1,
            FlowMode::CT => 2,
            FlowMode::Sleep => 7,
        }
        .to_string()
    }
}

impl QuoteStrings for &[&FlowTuple] {
    fn quote(&self) -> String {
        let mut s = '"'.to_string();
        for tuple in self.iter() {
            s.push_str(&tuple.quote());
            s.push(',');
        }
        s.pop();
        s.push('"');
        s
    }
}

macro_rules! params {
    ($($v:tt),+) => {
        vec!( $( $v.quote() ),+ ).join(", ")
    };
    () => {""};
}

macro_rules! gen_func {
    ($name:ident - $( $p:ident : $t:ty ),* ) => {
        impl Bulb {

            pub fn $name(&self, $($p : $t),*) -> Result<String, std::io::Error> {
                let message = craft_message(1, &stringify!($name), &params!($($p),*));
                self.send(&message)
            }

        }
    };
    ($name:ident) => { gen_func!($name - ); };
}

gen_func!(get_prop - properties: &[&str]);
gen_func!(set_ct_abx - ct_value: u64, effect: Effect, duration: u64);
gen_func!(set_rgb - rgb_value: u32, effect: Effect, duration: u64);
gen_func!(set_hsv - hue: u16, sat: u8, effect: Effect, duration: u64);
gen_func!(set_bright - brightness: u8, effect: Effect, duration: u64);
#[rustfmt::skip]
gen_func!(set_power - power: Power, effect: Effect, duration: u64, mode: Mode);
gen_func!(toggle);
gen_func!(set_default);
#[rustfmt::skip]
gen_func!(start_cf - count: u8, action: CfAction, flow_expression: &[&FlowTuple]);
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
gen_func!(bg_start_cf - count: u8, action: CfAction, flow_expression: &[&FlowTuple]);
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

impl Bulb {
    pub fn new(ip: &str, port: u16) -> Bulb {
        Bulb {
            ip: ip.to_string(),
            port,
        }
    }

    pub fn send_custom_message(
        &self,
        method: &str,
        params: &[&str],
    ) -> std::result::Result<String, std::io::Error> {
        let params = wrap_str_params(params);
        let message = craft_message_arr(1, method, params);
        self.send(&message)
    }

    fn send(&self, message: &str) -> std::result::Result<String, std::io::Error> {
        let mut stream = TcpStream::connect(format!("{}:{}", self.ip, self.port))?;

        stream.write_all(message.as_bytes())?;

        let reader = BufReader::new(stream);

        let mut line = String::new();

        let mut lines_iter = reader.lines();
        while !line.starts_with("{\"i") {
            match lines_iter.next() {
                Some(l) => line = l?,
                None => break,
            }
        }

        Ok(line)
    }
}

fn wrap_str_params(params: &[&str]) -> Vec<String> {
    params
        .iter()
        .map(|i| {
            if i.parse::<u64>().is_ok() {
                (*i).to_string()
            } else {
                i.quote()
            }
        })
        .collect()
}

fn craft_message(id: u64, method: &str, params: &str) -> String {
    format!(
        r#"{{ "id": {}, "method": "{}", "params": [{} ] }}"#,
        id, method, params
    ) + "\r\n"
}

fn craft_message_arr(id: u64, method: &str, params: Vec<String>) -> String {
    craft_message(id, method, &params.join(", "))
}
