# Yeelight CLI

A CLI to control your Yeelight smart lights.

## Install

```
cargo install yeelight-cli
```

## Usage

```
USAGE:
    yeelight-cli [OPTIONS] <address> <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --port <port>     [env: YEELIGHT_PORT=]  [default: 55443]

ARGS:
    <address>     [env: YEELIGHT_ADDR=]

SUBCOMMANDS:
    adjust            Adjust properties (Bright/CT/Color) (increase/decrease/circle)
    adjust-percent    Adjust properties (Bright/CT/Color) with perentage (-100~100)
    flow              Start color flow
    flow-stop         Stop color flow
    get               Get properties
    help              Prints this message or the help of the given subcommand(s)
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
