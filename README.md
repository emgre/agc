# Rust AGC

This work-in-progress software is a control-pulse accurate emulation of the
Bock II Apollo Guidance Computer developed in the 1960's for the Apollo
program.

It is mainly based on the [ND-1021042 document](https://archive.org/details/apollolunarexcuracel_0) published in February 1966.

## Usage

The only dependency is a stable version or [Rust](https://www.rust-lang.org/).

Simply run `cargo build` to build the project.

Unit tests can be executed with `cargo test`.

A TUI application is available to interactively run the emulator. To run it,
use `cargo run --bin agc-tui`. The right arrow key steps one clock cycle and
the escape key exits the application.

## Comparison with Virtual AGC

The [Virtual AGC](http://www.ibiblio.org/apollo/) project is a very complete
software emulation of the AGC. I sourced a lot of information from their work.

However, the emulation provided by Virtual AGC is precise to the instruction,
with a clock granularity to the subinstruction level. In other words, it
emulates the AGC instruction by instruction and accurately counts the number of
clock cycles per subinstruction.

The present implementation aims for finer granularity and precision. Therefore,
each subinstruction is divided into its 12 ticks and the emulation accurately
replicates each tick. It is therefore possible to stop the emulation in the
middle of a subinstruction and read the actual state of the all the registers,
both the "public" registers (those visible to the programmer) and the "private"
registers (those used internally by the computer).

## License

Since the AGC was developed at the MIT, it makes sense to release this code
under the MIT license. See [LICENSE.md](./LICENSE.md) for more details.

Copyright 2020 Émile Grégoire
