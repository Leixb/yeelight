# Yeelight

This project provides rust bindings for yeelight WiFi light interoperation spec.
[Yeelight_Inter-Operation_spec.pdf][1].

All the methods specified in the spec are implemented and named equally to the
aforementioned specification.

This intended as a API bindings for developers, if you want a cli tool to
control the lights take a look at [yeelight-cli][2] which uses this bindings (It
is also a good example of the usage and capabilities of this crate).

# Usage

The usage is quite straight forward, you can use the built-in bulb discovery
method to locate the Bulbs and connect to them. You can also connect directly
if you know the address beforehand. Once connected to the Bulb you can call the
various methods to change it's status.

## Bulb

The [Bulb] object represents an active connection to a singular light. All
operations are applied through calling methods on this object.

## Discovery

## Connection

### From discovered Bulbs

You can "upgrade" a [discover::DiscoveredBulb] to a [Bulb] by calling
[discover::DiscoveredBulb::connect].

### From address

You can connect using an address and port using [Bulb::connect] or create a
[Bulb] from an active TCP connection using [Bulb::attach] or [Bulb::attach_tokio].

## Basic operations

You can refer to the [Bulb] object documentation to view all the methods
available and their parameters.

## Music

## Flows

# Examples

# Implementation details

This crate is feature complete and can be used to control yeelight smart bulbs
with idiomatic Rust methods.

## Async

This crate uses `tokio` to manage all connections with the LEDs.

## Background light

The background light methods are separated from the foreground methods by
prefixing the methods with `bg_` like in the yeelight spec. Meaning that there
is [`Bulb::set_power`] for the foreground light and [`Bulb::bg_set_power`] for its background
counterpart and so on.

This may change in the future by adding a `isBackground` parameter to the
supported methods.

## Features

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

# Roadmap

Currently all main API features are implemented and only changes on the
usability of the API are planned. Nonetheless the planned changes may introduce
breaking changes.

- API features
    - [x] Implement all API functions
    - [x] Process bulb responses
    - [x] Discover Bulbs in the network
    - [x] Listen to Bulb notifications

- QoL improvements
    - [ ] Remove [CronType]?
    - [ ] Bulb response timeout
    - [ ] Change how background LEDs are treated
    - [ ] Merge [Effect] and `Duration` parameter
    - [ ] Make Music workflow more usable
    - [ ] Handle groups of Bulbs
- Testing
    - [ ] Cover all the main methods


[1]: https://www.yeelight.com/download/Yeelight_Inter-Operation_Spec.pdf
[2]: https://crates.io/crates/yeelight-cli
