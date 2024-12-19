# 3DS VC RNG Tool

This is a reference tool to demonstrate predicting the Virtual Console RNG. Currently, only Pokémon Crystal has been tested.

This is a quick, hacky, thrown together tool that could be severely improved code-wise and UX-wise.

## Usage

This tool requires the crystal/gen2 branch of PokeReader to obtain the required info.

This tool only works if you start on a DIV that hasn't been incremented between updating each RNG state halve. Test a few thousand advances. If the predictions are incorrect, start over. Eventually PokeReader will account for this automatically and it won't be an issue.

This tool has only been tested in an open dialogue.

This readme isn't guaranteed to stay up to date. To see the most up to date commands, run:

```
cargo run -- --help
```

Example usage at the time of writing:

```sh
# View all upcoming rands in a dialogue - works 100% when configured correctly
cargo run -- -d 13 -i 15127 -a 1728,1729,1c82,1c83,39d5,39d6 -s cc8f -S 388604 -E 560000 rng -l 10
# View potential shiny and max DV starters - does not work 100%
cargo run -- -d 13 -i 15127 -a 1728,1729,1c82,1c83,39d5,39d6 -s cc8f -S 388604 -E 560000 starter
```

Advancements (`-a`) will be a set of 6 numbers. Every other number should be 1 more than the previous number. If you have any set of numbers besides 6 after 16384 advances, reset Reader's DIV tracker.

There's always 2 pairs that end with an even number, and one pair that ends with an odd number. The even pairs are always 1370 apart. Any even/odd pair is 7507 apart.

In the example above, 0x39d5-0x1c82=7507 and 0x1c82-0x1728=1370 (0x1728-0x39d5)%0x4000=7507.
