# Yeelight
[![built with nix](https://img.shields.io/static/v1?logo=nixos&logoColor=white&label=&message=Built%20with%20Nix&color=41439a)](https://builtwithnix.org "built with nix")
[![build](https://github.com/Leixb/yeelight/actions/workflows/rust.yml/badge.svg)](https://github.com/Leixb/yeelight/actions/workflows/rust.yml)

This project provides Rust bindings for the [Yeelight Wi-Fi Light Interoperation Specification][1].

All the methods specified in the spec are implemented and named equally to the
aforementioned specification.

This project can be used both as a binary crate or as library with API bindings
for developers. The binary crate provides a CLI tool to control lights (this
replaces [yeelight-cli][2])

## Table of Contents
- [CLI Usage](#cli-usage)
- [Library Usage](#library-usage)
  - [Bulb](#bulb)
  - [Discovery](#discovery)
  - [Connection](#connection)
    - [From discovered bulbs](#from-discovered-bulbs)
    - [From address](#from-address)
  - [Basic Operations](#basic-operations)
  - [Music Mode](#music-mode)
  - [Flows](#flows)
- [Implementation](#implementation-details)
  - [Async](#async)
  - [Background light](#background-light)
  - [Features](#features)
- [Examples](#examples)
  - [Collatz](#collatz)
  - [Flow](#flow)
  - [Music](#music)
  - [Notification](#notification)
  - [Properties](#properties)
  - [Toggle](#toggle)
- [Roadmap](#roadmap)
  - [API features](#api-features)
  - [Quality of life](#quality-of-life)
  - [Testing](#testing)
- [Troubleshooting](#troubleshooting)
  - [Connection refused](#connection-refused)
  - [Invalid params](#invalid-params)


## Xiaomi warning

**!!! Do NOT update your light firmware of Xiaomi branded products !!!**

Starting January 2021, Xiaomi branded smart lights lost their LAN control
functionality when updating firmware with no option to rollback.
[see](https://twitter.com/home_assistant/status/1376789260611637251)

## CLI Usage

You can run a cli to control lights from by installing it with cargo or using
cargo run. The program name will be `yeelight`:

```bash
cargo install yeelight
yeelight --help # or cargo run -- --help
```

There are commands for all yeelight API specs:

```
yeelight 0.4.0
A CLI to control your Yeelight smart lights.

USAGE:
    yeelight [OPTIONS] [address] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --port <port>           [env: YEELIGHT_PORT=]  [default: 55443]
    -t, --timeout <timeout>     [env: YEELIGHT_TIMEOUT=]  [default: 5000]

ARGS:
    <address>     [env: YEELIGHT_ADDR=]  [default: NULL]

SUBCOMMANDS:
    adjust            Adjust properties (Bright/CT/Color) (increase/decrease/circle)
    adjust-percent    Adjust properties (Bright/CT/Color) with percentage (-100~100)
    discover          
    flow              Start color flow
    flow-stop         Stop color flow
    get               Get properties
    help              Prints this message or the help of the given subcommand(s)
    listen            Listen to notifications from lamp
    music-connect     Connect to music TCP stream
    music-stop        Stop music mode
    off               Turn off light
    on                Turn on light
    preset            Presets
    set               Set values
    timer             Start timer
    timer-clear       Clear current timer
    timer-get         Get remaining minutes for timer
    toggle            Toggle light
```

### Specifying the lamp

The only required argument is `<address>` which can be either an IP address, the
name of a lamp, or the value `all`:

- If an IP address is provided, `yeelight`
  will attempt to connect with a bulb on that address, if `timeout` milliseconds
  pass and a connection cannot be established, it will fail.
- If a name is provided, `yeelight` will launch a discovery message and wait
  for a bulb with the given name to respond. If such bulb is not found in `timeout`
  milliseconds, it will fail.
- If the special keyword `all` is given, the command will be run on all the
  bulbs that respond to a discovery message on the network in the given `timeout`
  window.

Additionally, you can set the environment variable `YEELIGHT_ADDR` to specify
the default address if none is provided.

When running the `discovery` command, there is no need to specify the address,
in all other cases, an address must be provided.

### Subcommands

Details on the functionality and options of each command can be seen by issuing
`--help` on each subcommand.

## Library Usage

The usage is quite straight forward, you can use the built-in bulb discovery
method to locate the Bulbs and connect to them. You can also connect directly
if you know the address beforehand. Once connected to the Bulb you can call the
various methods to change its status.

### Bulb

The [`Bulb`] object represents an active connection to a singular light. All
operations are applied through calling methods on this object.

### Discovery

### Connection

#### From discovered Bulbs

You can "upgrade" a [`discover::DiscoveredBulb`] to a [`Bulb`] by calling
[`discover::DiscoveredBulb::connect`].

#### From address

You can connect using an address and port using [`Bulb::connect`] or create a
[`Bulb`] from an active TCP connection using [`Bulb::attach`] or [`Bulb::attach_tokio`].

### Basic operations

You can refer to the `Bulb` object documentation to view all the methods
available and their parameters.

### Music Mode
Music mode essentially upgrades the existing connection to a reverse one (the bulb connects to the library), allowing you to send commands without being rate-limited.

Starting music mode will start a new listening socket, tell the bulb to connect to that, and then close the old connection. Use the IP address of the machine where the library/your project runs as the host. (e.g., `192.168.5.23`).

#### Note
Make sure to use a 1.X version of Tokio for this to work.

### Flows

## Implementation details

This crate is feature complete and can be used to control Yeelight smart bulbs
with idiomatic Rust methods.

### Async

This crate uses `tokio` to manage all connections with the LEDs.

### Background light

The background light methods are separated from the foreground methods by
prefixing the methods with `bg_` like in the yeelight spec. Meaning that there
is [`Bulb::set_power`] for the foreground light and [`Bulb::bg_set_power`] for its background
counterpart and so on.

This may change in the future by adding a `isBackground` parameter to the
supported methods.

### Features

By default this crate uses all it's features. In some cases where the space is
most crucial you can compile this crate without some of them to reduce it's
impact.

Currently there are only 2 different features:

- "from-str": This enables parsing responses from the bulb and addresses from
  strings.
- "discovery": This enables Bulb discovery.

In the future there may be another feature removing tokio altogether allowing
to use the crate in minimal systems. However you can use the 0.2 version of the
crate which does not have async.

## Examples
All examples can also be found in the `examples` directory.

### Collatz
```rust
use std::{thread, time::Duration};

use yeelight::{Bulb, Effect, Mode, Power, Properties, Property};

// This program is meant to demonstrate some examples of commands and how to read the results turns
// on the bulb, changes the brightness following the collatz sequence (mod 100) 10 times waiting 1
// second each, and then sets the color to red over 10 seconds.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::init();

    let mut bulb = Bulb::connect("192.168.1.204", 55443).await?;

    // Turn on the bulb
    println!(
        "Response: {:?}",
        bulb.set_power(
            Power::On,
            Effect::Sudden,
            Duration::from_secs(1),
            Mode::Normal
        )
        .await?
    );

    // Define flow array
    let props = Properties(vec![
        Property::Power,
        Property::Bright,
        Property::CT,
        Property::RGB,
    ]);

    for _ in 1..10u8 {
        let response = bulb.get_prop(&props).await?.unwrap();
        let brightness = response[1].parse::<u32>()?;

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
        let response = bulb
            .set_bright(brightness, Effect::Smooth, Duration::from_secs(1))
            .await?;
        eprintln!("Response: {:?}", response);

        thread::sleep(Duration::from_secs(1));
    }

    // Set bulb to pure red over 10 seconds
    bulb.set_rgb(0xff_00_00, Effect::Smooth, Duration::from_secs(1))
        .await?;
    Ok(())
}
```

### Flow
```rust
use std::time::Duration;

use yeelight::{Bulb, CfAction, Effect, FlowExpresion, FlowTuple, Mode, Power};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::init();

    let my_bulb_ip = "192.168.1.204";
    let mut bulb = Bulb::connect(my_bulb_ip, 55443).await?;

    // Turn on the bulb
    let response = bulb
        .set_power(Power::On, Effect::Sudden, Duration::new(0, 0), Mode::Normal)
        .await?;
    println!("response: {:?}", response);

    // Define flow array
    let flow = FlowExpresion(vec![
        FlowTuple::ct(Duration::from_millis(500), 3000, 100),
        FlowTuple::sleep(Duration::from_millis(1500)),
        FlowTuple::ct(Duration::from_millis(500), 5000, 100),
        FlowTuple::sleep(Duration::from_millis(1500)),
        FlowTuple::ct(Duration::from_millis(500), 2600, 100),
        FlowTuple::sleep(Duration::from_millis(1500)),
    ]);
    // Send flow command
    let response = bulb.start_cf(10, CfAction::Stay, flow).await?;
    println!("response: {:?}", response);
    Ok(())
}
```

### Music
```rust
use std::time::Duration;

use yeelight::{Bulb, Effect, Power, Mode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::init();

    let my_bulb_ip = "192.168.1.200";
    let my_computer_ip = "192.168.1.23";

    let mut bulb = Bulb::connect(my_bulb_ip, 0).await?;
    let mut music_conn = bulb.start_music(my_computer_ip).await?;

    let sleep_duration = Duration::from_millis(300);
    let no_duration = Duration::from_millis(0);

    bulb.set_power(Power::On, Effect::Sudden, no_duration, Mode::Normal).await?;

    for _ in 0..60 { 
        std::thread::sleep(sleep_duration);
        music_conn.set_rgb(0x00ff00, Effect::Sudden, no_duration).await?;
        std::thread::sleep(sleep_duration);
        music_conn.set_rgb(0x0000ff, Effect::Sudden, no_duration).await?;
        std::thread::sleep(sleep_duration);
        music_conn.set_rgb(0xff0000, Effect::Sudden, no_duration).await?;
    }

    drop(music_conn);

    Ok(())
}
```

### Notification
```rust
use yeelight::Bulb;

#[tokio::main]
async fn main() {

    env_logger::init();

    let my_bulb_ip = "192.168.1.200";
    let mut bulb = Bulb::connect(my_bulb_ip, 55443)
        .await
        .expect("Connection failed");
    if let Some(response) = bulb.toggle().await.expect("Error") {
        for v in response.iter() {
            println!("{}", v);
        }
    }
}
```

### Properties
```rust
use std::time::Duration;

use yeelight::{Bulb, Effect, Mode, Power, Properties, Property};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    env_logger::init();

    let my_bulb_ip = "192.168.1.204";
    let mut bulb = Bulb::connect(my_bulb_ip, 55443).await?;

    // Turn on the bulb
    println!(
        "Response: {:?}",
        bulb.set_power(
            Power::On,
            Effect::Sudden,
            Duration::from_millis(0),
            Mode::Normal
        )
        .await?
    );

    // Define flow array
    let props = Properties(vec![
        Property::Power,
        Property::Bright,
        Property::CT,
        Property::RGB,
    ]);
    // Send flow command
    println!("Response: {:?}", bulb.get_prop(&props).await?);
    println!(
        "Response: {:?}",
        bulb.set_rgb(122, Effect::Smooth, Duration::from_millis(500))
            .await?
    );
    Ok(())
}
```

### Toggle
```rust
use yeelight::Bulb;

#[tokio::main]
async fn main() {

    env_logger::init();

    let mut bulb = Bulb::connect(my_bulb_ip, 55443)
        .await
        .expect("Connection failed");
    if let Some(response) = bulb.toggle().await.expect("Error") {
        for v in response.iter() {
            println!("{}", v);
        }
    }
}
```

## Roadmap

Currently all main API features are implemented and only changes on the
usability of the API are planned. Nonetheless the planned changes may introduce
breaking changes.

### API features
- [x] Implement all API functions
- [x] Process bulb responses
- [x] Discover Bulbs in the network
- [x] Listen to Bulb notifications

### Quality of life
- [ ] Remove [CronType]?
- [x] Bulb response timeout
- [ ] Change how background LEDs are treated
- [ ] Merge [Effect] and `Duration` parameter
- [x] Make Music workflow more usable
- [ ] Handle groups of Bulbs
### Testing
- [ ] Cover all the main methods


[1]: https://web.archive.org/web/20210131152550/https://www.yeelight.com/download/Yeelight_Inter-Operation_Spec.pdf
[2]: https://crates.io/crates/yeelight-cli

## Troubleshooting

### Connection Refused
`Error: Os { code: 111, kind: ConnectionRefused, message: "Connection refused" }`
- Make sure to turn off running VPNs.

### Invalid Params
`Error: ErrResponse(-5001, "invalid params")`
- Turn off Music Flow on the Yeelight/Mi Home app.

### Rate limit
- Turn on [music mode](#music-mode)
