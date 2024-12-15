# 3DS VC RNG Tool

This is a reference tool to demonstrate predicting the Virtual Console RNG. Currently, only Pokémon Crystal has been tested.

This is a quick, hacky, thrown together tool that could be severely improved code-wise and UX-wise.

## Usage

This tool requires the crystal/gen2 branch of PokeReader to obtain the required info.

This tool only works if you start on a DIV that hasn't been incremented between updating each RNG state halve. If the prediction is incorrect, advance a time or two and try again.

This readme isn't guaranteed to stay up to date. To see the most up to date commands, run:

```
cargo run -- --help
```

Example usage at the time of writing:

```
cargo run -- -d 13 -i 15127 -a 1728,1729,1c82,1c83,39d5,39d6 -s cc8f -l 10 -S 388604 -E 560000
```

Advancements (`-a`) will be a set of 6 numbers. Every other number should be 1 more than the previous number. If you have any set of numbers besides 6 after 16384 advances, reset Reader's DIV tracker.

One set of advancements should be 1370 apart, and the other should be 7507 apart. The order doesn't matter. In the example above, 0x39d5-0x1c82=7507 and 0x1c82-0x1728=1370.
