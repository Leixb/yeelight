mod presets;

use std::{time::Duration, collections::HashSet};

use structopt::{
    clap::{AppSettings, ArgGroup},
    StructOpt,
};

use tokio::sync::mpsc;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "yeelight-cli",
    about = "A CLI to control your Yeelight smart lights."
)]
#[structopt(global_setting = AppSettings::ColoredHelp)]
struct Options {
    #[structopt(env = "YEELIGHT_ADDR", default_value = "NULL")]
    address: String,
    #[structopt(short, long, default_value = "55443", env = "YEELIGHT_PORT")]
    port: u16,
    #[structopt(short, long, default_value = "5000", env = "YEELIGHT_TIMEOUT")]
    timeout: u64,
    #[structopt(subcommand)]
    subcommand: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(about = "Get properties")]
    Get {
        #[structopt(possible_values = &yeelight::Property::variants(), case_insensitive = true)]
        #[structopt(required = true)]
        properties: Vec<yeelight::Property>,
    },
    #[structopt(about = "Toggle light")]
    #[structopt(group = ArgGroup::with_name("light"))]
    Toggle {
        #[structopt(long, group = "light", help = "Perform action on all lights of device")]
        dev: bool,
        #[structopt(long, group = "light", help = "Perform action on background light")]
        bg: bool,
    },
    #[structopt(about = "Turn on light")]
    On {
        #[structopt(possible_values = &yeelight::Effect::variants(), case_insensitive = true)]
        #[structopt(short, long, default_value = "Smooth")]
        effect: yeelight::Effect,
        #[structopt(short, long, default_value = "500")]
        duration: u64,
        #[structopt(possible_values = &yeelight::Mode::variants(), case_insensitive = true)]
        #[structopt(short, long, default_value = "Normal")]
        mode: yeelight::Mode,
        #[structopt(long, help = "Perform action on background light")]
        bg: bool,
    },
    #[structopt(about = "Turn off light")]
    Off {
        #[structopt(possible_values = &yeelight::Effect::variants(), case_insensitive = true)]
        #[structopt(short, long, default_value = "Smooth")]
        effect: yeelight::Effect,
        #[structopt(short, long, default_value = "500")]
        duration: u64,
        #[structopt(possible_values = &yeelight::Mode::variants(), case_insensitive = true)]
        #[structopt(short, long, default_value = "Normal")]
        mode: yeelight::Mode,
        #[structopt(long, help = "Perform action on background light")]
        bg: bool,
    },
    #[structopt(about = "Start timer")]
    Timer { minutes: u64 },
    #[structopt(about = "Clear current timer")]
    TimerClear,
    #[structopt(about = "Get remaining minutes for timer")]
    TimerGet,
    #[structopt(about = "Set values")]
    Set {
        #[structopt(flatten)]
        property: Prop,
        #[structopt(possible_values = &yeelight::Effect::variants(), case_insensitive = true)]
        #[structopt(short, long, default_value = "Smooth")]
        effect: yeelight::Effect,
        #[structopt(short, long, default_value = "500")]
        duration: u64,
    },
    #[structopt(about = "Start color flow")]
    Flow {
        expression: yeelight::FlowExpresion,
        #[structopt(default_value = "0")]
        count: u8,
        #[structopt(possible_values = &yeelight::CfAction::variants(), case_insensitive = true)]
        #[structopt(default_value = "Recover")]
        action: yeelight::CfAction,
        #[structopt(long, help = "Perform action on background light")]
        bg: bool,
    },
    #[structopt(about = "Stop color flow")]
    FlowStop {
        #[structopt(long, help = "Perform action on background light")]
        bg: bool,
    },
    #[structopt(about = "Adjust properties (Bright/CT/Color) (increase/decrease/circle)")]
    Adjust {
        #[structopt(possible_values = &yeelight::Prop::variants(), case_insensitive = true)]
        property: yeelight::Prop,
        #[structopt(possible_values = &yeelight::AdjustAction::variants(), case_insensitive = true)]
        action: yeelight::AdjustAction,
        #[structopt(long, help = "Perform action on background light")]
        bg: bool,
    },
    #[structopt(about = "Adjust properties (Bright/CT/Color) with perentage (-100~100)")]
    #[structopt(setting = AppSettings::AllowNegativeNumbers)]
    AdjustPercent {
        #[structopt(possible_values = &yeelight::Prop::variants(), case_insensitive = true)]
        property: yeelight::Prop,
        percent: i8,
        #[structopt(default_value = "500")]
        duration: u64,
        #[structopt(long, help = "Perform action on background light")]
        bg: bool,
    },
    #[structopt(about = "Connect to music TCP stream")]
    MusicConnect { host: String, port: u16 },
    #[structopt(about = "Stop music mode")]
    MusicStop,
    #[structopt(about = "Presets")]
    Preset {
        #[structopt(possible_values = &presets::Preset::variants(), case_insensitive = true)]
        preset: presets::Preset,
    },
    #[structopt(about = "Listen to notifications from lamp")]
    Listen,
    Discover{
        #[structopt(long, default_value = "5000")]
        duration: u64,
    },
}

#[derive(Debug, StructOpt)]
enum Prop {
    Power {
        #[structopt(possible_values = &yeelight::Power::variants(), case_insensitive = true)]
        power: yeelight::Power,
        #[structopt(possible_values = &yeelight::Mode::variants(), case_insensitive = true)]
        #[structopt(default_value = "Normal")]
        mode: yeelight::Mode,
        #[structopt(long, help = "Perform action on background light")]
        bg: bool,
    },
    CT {
        color_temperature: u16,
        #[structopt(long, help = "Perform action on background light")]
        bg: bool,
    },
    RGB {
        rgb_value: u32,
        #[structopt(long, help = "Perform action on background light")]
        bg: bool,
    },
    HSV {
        hue: u16,
        #[structopt(default_value = "100")]
        sat: u8,
        #[structopt(long, help = "Perform action on background light")]
        bg: bool,
    },
    Bright {
        brightness: u8,
        #[structopt(long, help = "Perform action on background light")]
        bg: bool,
    },
    Name {
        name: String,
    },
    Scene {
        #[structopt(possible_values = &yeelight::Class::variants(), case_insensitive = true)]
        class: yeelight::Class,
        val1: u64,
        #[structopt(default_value = "100")]
        val2: u64,
        #[structopt(default_value = "100")]
        val3: u64,
        #[structopt(long, help = "Perform action on background light")]
        bg: bool,
    },
    Default {
        #[structopt(long, help = "Perform action on background light")]
        bg: bool,
    },
}

macro_rules! sel_bg {
    ($obj:tt.$fn:ident ($($p:expr),*) || $fn_bg:ident if $var:tt ) => (
        if $var {
            $obj.$fn_bg($($p),*).await
        } else {
            $obj.$fn($($p),*).await
        }
    );
}

#[tokio::main]
async fn main() {
    let opt = Options::from_args();

    if let Command::Discover{duration} = opt.subcommand {

        let search = async {
            let mut channel = yeelight::discover::find_bulbs().await.unwrap();
            let dash = "-".to_owned();
            let mut found = HashSet::new();

            while let Some(dbulb) = channel.recv().await {
                if found.contains(&dbulb) {
                    continue;
                }
                let name = dbulb.properties.get("name").unwrap_or(&dash);
                let location = dbulb.properties.get("Location").unwrap_or(&dash);
                // remove prefix
                let location = location.trim_start_matches("yeelight://");
                println!("{} ({})", &location, &name);
                found.insert(dbulb);
            }
        };

        // if duration if == 0 do not timeout
        if duration > 0 {
            let _ = tokio::time::timeout(Duration::from_millis(duration), search).await;
        } else {
            search.await;
        }

        return
    }

    if opt.address == "NULL" {
        structopt::clap::Error::with_description("No address specified (use --help for more info)", structopt::clap::ErrorKind::MissingRequiredArgument).exit();
    }

    let mut bulb = tokio::time::timeout(Duration::from_secs(opt.timeout), async {
        return yeelight::Bulb::connect(&opt.address, opt.port).await.unwrap();
    }).await.unwrap();

    let response = match opt.subcommand {
        Command::Toggle{bg, dev} => {
            match (bg, dev) {
                (true, _) => bulb.bg_toggle().await,
                (_, true) => bulb.dev_toggle().await,
                _ => bulb.toggle().await,
            }
        },
        Command::On {
            effect,
            duration,
            mode,
            bg,
        } => sel_bg!(bulb.set_power(yeelight::Power::On, effect, Duration::from_millis(duration), mode) || bg_set_power if bg),
        Command::Off {
            effect,
            duration,
            mode,
            bg,
        } => sel_bg!(bulb.set_power(yeelight::Power::Off, effect, Duration::from_millis(duration), mode) || bg_set_power if bg),
        Command::Get { properties } => bulb.get_prop(&yeelight::Properties(properties)).await,
        Command::Set {
            property,
            effect,
            duration,
        } => match property {
            Prop::Power { power, mode, bg} => sel_bg!(bulb.set_power(power, effect, Duration::from_millis(duration), mode) || bg_set_power if bg),
            Prop::CT { color_temperature, bg} => sel_bg!(bulb.set_ct_abx(color_temperature, effect, Duration::from_millis(duration)) || bg_set_ct_abx if bg),
            Prop::RGB { rgb_value, bg} => sel_bg!(bulb.set_rgb(rgb_value, effect, Duration::from_millis(duration)) || bg_set_rgb if bg),
            Prop::HSV { hue, sat, bg} => sel_bg!(bulb.set_hsv(hue, sat, effect, Duration::from_millis(duration)) || bg_set_hsv if bg),
            Prop::Bright { brightness, bg} => sel_bg!(bulb.set_bright(brightness, effect, Duration::from_millis(duration)) || bg_set_bright if bg),
            Prop::Name { name } => bulb.set_name(&name).await,
            Prop::Scene {
                class,
                val1,
                val2,
                val3,
                bg,
            } => sel_bg!(bulb.set_scene(class, val1, val2, val3) || bg_set_scene if bg),
            Prop::Default{bg} => sel_bg!(bulb.set_default() || bg_set_default if bg),
        },
        Command::Timer { minutes } => bulb.cron_add(yeelight::CronType::Off, minutes).await,
        Command::TimerClear => bulb.cron_del(yeelight::CronType::Off).await,
        Command::TimerGet => bulb.cron_get(yeelight::CronType::Off).await,
        Command::Flow {
            count,
            action,
            expression,
            bg,
        } => sel_bg!(bulb.start_cf(count, action, expression) || bg_start_cf if bg),
        Command::FlowStop { bg } => sel_bg!(bulb.stop_cf() || bg_stop_cf if bg),
        Command::Adjust { action, property, bg } => sel_bg!(bulb.set_adjust(action, property) || bg_set_adjust if bg),
        Command::AdjustPercent {
            property,
            percent,
            duration,
            bg,
        } => match property {
            yeelight::Prop::Bright => sel_bg!(bulb.adjust_bright(percent, Duration::from_millis(duration)) || bg_adjust_bright if bg),
            yeelight::Prop::Color => sel_bg!(bulb.adjust_color(percent, Duration::from_millis(duration)) || bg_adjust_color if bg),
            yeelight::Prop::CT => sel_bg!(bulb.adjust_ct(percent, Duration::from_millis(duration)) || bg_adjust_ct if bg),
        },
        Command::MusicConnect { host, port } => {
            bulb.set_music(yeelight::MusicAction::On, &host, port).await
        }
        Command::MusicStop => bulb.set_music(yeelight::MusicAction::Off, &"".to_string(), 0).await,
        Command::Preset{ preset } => presets::apply(bulb, preset).await,
        Command::Listen => {
            let (sender, mut recv) = mpsc::channel(10);

            bulb.set_notify(sender).await;

            while let Some(yeelight::Notification(i)) = recv.recv().await {
                for (k, v) in i.iter() {
                    println!("{} {}", k, v);
                }
            }
            Ok(None)
        }
        Command::Discover{duration: _} => Ok(None)
    }
    .unwrap();

    if let Some(result) = response {
        result.iter().for_each(|x| {
            if x != "ok" {
                println!("{}", x)
            }
        });
    }
}
