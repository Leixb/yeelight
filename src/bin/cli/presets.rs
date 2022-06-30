use yeelight::{BulbError, FlowExpresion, FlowTuple, Response};

use std::time::Duration;

use structopt::clap::arg_enum;

arg_enum! {
    #[derive(Debug, Clone)]
    pub enum Preset {
        Candle,
        Reading,
        NightReading,
        CosyHome,
        Romantic,
        Birthday,
        DateNight,
        Teatime,
        PcMode,
        Concentration,
        Movie,
        Night,
        Notify,
        Notify2,

        PulseRed,
        PulseBlue,
        PulseGreen,

        Red,
        Green,
        Blue,

        Police,
        Police2,
        Disco,
        Temp,
    }
}

enum PresetValue {
    RGB(u32, u8),
    HSV(u16, u8, u8),
    CT(u16, u8),
    Flow(yeelight::FlowExpresion, u8, yeelight::CfAction),
}

pub async fn apply(bulb: yeelight::Bulb, preset: Preset) -> Result<Option<Response>, BulbError> {
    use Preset::*;
    let red = 0xFF_00_00;
    let green = 0x00_FF_00;
    let blue = 0x00_00_FF;
    match preset {
        Candle => send(bulb, candle()).await,
        Reading => send(bulb, reading()).await,
        NightReading => send(bulb, night_reading()).await,
        CosyHome => send(bulb, cosy_home()).await,
        Romantic => send(bulb, romantic()).await,
        Birthday => send(bulb, birthday()).await,
        DateNight => send(bulb, date_night()).await,
        Teatime => send(bulb, teatime()).await,
        PcMode => send(bulb, pc_mode()).await,
        Concentration => send(bulb, concentration()).await,
        Movie => send(bulb, movie()).await,
        Night => send(bulb, night()).await,
        Notify => send(bulb, notify()).await,
        Notify2 => send(bulb, notify2()).await,

        Red => send(bulb, PresetValue::RGB(red, 100)).await,
        Green => send(bulb, PresetValue::RGB(green, 100)).await,
        Blue => send(bulb, PresetValue::RGB(blue, 100)).await,

        PulseRed => send(bulb, pulse(red, 100, 250)).await,
        PulseGreen => send(bulb, pulse(green, 100, 250)).await,
        PulseBlue => send(bulb, pulse(blue, 100, 250)).await,
        Police => send(bulb, police(100)).await,
        Police2 => send(bulb, police2(100)).await,
        Disco => send(bulb, disco(120)).await,
        Temp => send(bulb, temp(2600, 5000, 100)).await,
    }
}

async fn send(
    mut bulb: yeelight::Bulb,
    preset: PresetValue,
) -> Result<Option<Response>, BulbError> {
    match preset {
        PresetValue::Flow(expression, count, action) => {
            bulb.start_cf(count, action, expression).await
        }
        PresetValue::RGB(color, bright) => {
            bulb.set_scene(yeelight::Class::Color, color.into(), bright.into(), 0)
                .await
        }
        PresetValue::HSV(hue, sat, bright) => {
            bulb.set_scene(yeelight::Class::HSV, hue.into(), sat.into(), bright.into())
                .await
        }
        PresetValue::CT(ct, bright) => {
            bulb.set_scene(yeelight::Class::CT, ct.into(), bright.into(), 0)
                .await
        }
    }
}

fn disco(bpm: u64) -> PresetValue {
    let duration = Duration::from_millis(1000/ bpm);
    let expr = FlowExpresion(vec![
        FlowTuple::rgb(duration, 0xFF_00_00, 100),
        FlowTuple::rgb(duration, 0xFF_00_00, 1),
        FlowTuple::rgb(duration, 0x80_FF_00, 100),
        FlowTuple::rgb(duration, 0x80_FF_00, 1),
        FlowTuple::rgb(duration, 0x00_FF_FF, 100),
        FlowTuple::rgb(duration, 0x00_FF_FF, 1),
        FlowTuple::rgb(duration, 0x80_00_FF, 100),
        FlowTuple::rgb(duration, 0x80_00_FF, 1),
    ]);
    PresetValue::Flow(expr, 0, yeelight::CfAction::Stay)
}

fn temp(a: u32, b: u32, brightness: i8) -> PresetValue {
    let duration = Duration::from_millis(40_000);
    let expr = FlowExpresion(vec![
        FlowTuple::ct(duration, a, brightness),
        FlowTuple::ct(duration, b, brightness),
    ]);
    PresetValue::Flow(expr, 0, yeelight::CfAction::Stay)
}

fn pulse(rgb: u32, brightness: i8, duration: u64) -> PresetValue {
    let duration = Duration::from_millis(duration);
    let expr = FlowExpresion(vec![
        FlowTuple::rgb(duration, rgb, brightness),
        FlowTuple::rgb(duration, rgb, 1),
    ]);
    PresetValue::Flow(expr, 2, yeelight::CfAction::Recover)
}

fn police(brightness: i8) -> PresetValue {
    let duration = Duration::from_millis(300);
    let (red, blue) = (0xFF_00_00, 0x00_00_FF);
    let expr = FlowExpresion(vec![
        FlowTuple::rgb(duration, red, brightness),
        FlowTuple::rgb(duration, blue, brightness),
    ]);
    PresetValue::Flow(expr, 0, yeelight::CfAction::Stay)
}

fn police2(brightness: i8) -> PresetValue {
    let duration = Duration::from_millis(300);
    let (red, blue) = (0xFF_00_00, 0x00_00_FF);
    let expr = FlowExpresion(vec![
        FlowTuple::rgb(duration, red, brightness),
        FlowTuple::rgb(duration, red, 1),
        FlowTuple::rgb(duration, red, brightness),
        FlowTuple::sleep(duration),
        FlowTuple::rgb(duration, blue, brightness),
        FlowTuple::rgb(duration, blue, 1),
        FlowTuple::rgb(duration, blue, brightness),
        FlowTuple::sleep(duration),
    ]);
    PresetValue::Flow(expr, 0, yeelight::CfAction::Stay)
}

fn candle() -> PresetValue {
    let ct = 2700;
    let expr = FlowExpresion(vec![
        FlowTuple::ct(Duration::from_millis(800), ct, 50),
        FlowTuple::ct(Duration::from_millis(800), ct, 30),
        FlowTuple::ct(Duration::from_millis(1200), ct, 80),
        FlowTuple::ct(Duration::from_millis(800), ct, 60),
        FlowTuple::ct(Duration::from_millis(1200), ct, 90),
        FlowTuple::ct(Duration::from_millis(2400), ct, 50),
        FlowTuple::ct(Duration::from_millis(1200), ct, 80),
        FlowTuple::ct(Duration::from_millis(800), ct, 60),
        FlowTuple::ct(Duration::from_millis(400), ct, 70),
    ]);
    PresetValue::Flow(expr, 0, yeelight::CfAction::Stay)
}
fn reading() -> PresetValue {
    PresetValue::CT(3500, 100)
}
fn night_reading() -> PresetValue {
    PresetValue::CT(4000, 40)
}

fn cosy_home() -> PresetValue {
    PresetValue::CT(2700, 80)
}

fn romantic() -> PresetValue {
    let expr = FlowExpresion(vec![
        FlowTuple::rgb(Duration::from_millis(4000), 0x59_15_6D, 1),
        FlowTuple::rgb(Duration::from_millis(4000), 0x66_14_2A, 1),
    ]);
    PresetValue::Flow(expr, 0, yeelight::CfAction::Stay)
}

fn birthday() -> PresetValue {
    let expr = FlowExpresion(vec![
        FlowTuple::rgb(Duration::from_millis(1996), 0xDC_50_19, 80),
        FlowTuple::rgb(Duration::from_millis(1996), 0xDC_78_1E, 80),
        FlowTuple::rgb(Duration::from_millis(1996), 0xAA_32_14, 80),
    ]);
    PresetValue::Flow(expr, 0, yeelight::CfAction::Stay)
}

fn date_night() -> PresetValue {
    PresetValue::HSV(24, 100, 50)
}

fn teatime() -> PresetValue {
    PresetValue::CT(3000, 50)
}

fn pc_mode() -> PresetValue {
    PresetValue::CT(2700, 30)
}
fn concentration() -> PresetValue {
    PresetValue::CT(5000, 100)
}

fn movie() -> PresetValue {
    PresetValue::HSV(240, 60, 50)
}

fn night() -> PresetValue {
    PresetValue::HSV(36, 100, 1)
}

fn notify() -> PresetValue {
    let duration = Duration::from_millis(300);
    let temp = 5000;
    let expr = FlowExpresion(vec![
        FlowTuple::ct(duration, temp, 100),
        FlowTuple::ct(duration, temp, 1),
        FlowTuple::ct(duration, temp, 100),
        FlowTuple::ct(duration, temp, 1),
        FlowTuple::ct(duration, temp, 100),
        FlowTuple::ct(duration, temp, 1),
    ]);
    let len = &expr.0.len();
    PresetValue::Flow(expr, *len as u8, yeelight::CfAction::Recover)
}

fn notify2() -> PresetValue {
    let duration = Duration::from_millis(200);
    let temp = 5000;
    let expr = FlowExpresion(vec![
        FlowTuple::ct(duration, temp, 100),
        FlowTuple::ct(duration, temp, 1),
        FlowTuple::ct(duration, temp, 100),
        FlowTuple::ct(duration, temp, 1),
    ]);
    let len = &expr.0.len();
    PresetValue::Flow(expr, *len as u8, yeelight::CfAction::Recover)
}
