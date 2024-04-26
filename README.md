# rust-arduino-demo

============

Rust project for the _Arduino Uno_. This is a simple demo using the
built in ADC to provide data for configurable IIR low and high pass
filters of the first order.

Channel 0 (A0) is used to convert an analog signal which is then
filtered and sent to Port B and D via PD2 to PD7 and PB0 to PB2.
The filter can be switched via PB5 and the frequency can be set
via PC1 to PC5 (Pins A1 to A5).

PB3 and PB4 provide input and output signalling for a latch.

## Build Instructions

1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware. If you want the hex files for
simulation run `cargo build` a second time. This step is only needed once.
Hex files will be produced thereafter with every build unless you run `cargo clean`
which then requires you to run twice again.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## Produced Output

You will find the hex files for simulation in the `avr-atmega328p` directory of the
`target` directory.

## License

Licensed under either of

- Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
