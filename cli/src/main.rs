use clap::Parser;
use std::num::ParseIntError;
use vc_rng_lib::{generate_rng_states, generate_starters, Filter, PokeOptions, RandOptions};

fn parse_u16_hex(input: &str) -> Result<u16, ParseIntError> {
    u16::from_str_radix(input, 16)
}

fn limit_index_range(input: &str) -> Result<usize, String> {
    let input = input.parse::<usize>().map_err(|e| e.to_string())?;
    // Laziness
    if !(..0x4000).contains(&input) {
        Err("Index must be between 0 and 16383".to_string())
    } else {
        Ok(input)
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Reference 3ds VC Rng Tool.
/// Currently only works with Crystal.
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[arg(short, long, value_parser = parse_u16_hex)]
    adiv: u8,
    #[arg(short, long, value_parser = parse_u16_hex)]
    sdiv: u8,
    #[arg(short, long, value_parser = limit_index_range)]
    adiv_index: usize,
    #[arg(short, long, value_parser = limit_index_range)]
    sdiv_index: usize,
    #[arg(short, long, value_parser = parse_u16_hex)]
    state: u16,
    #[arg(short = 'S', long)]
    start_advance: usize,
    #[arg(short = 'c', long)]
    advance_count: usize,
}

#[derive(Parser)]
enum Command {
    Rng,
    Starter,
}

fn main() {
    let opts = Cli::parse();
    let end_advance = opts.start_advance + opts.advance_count;

    match opts.command {
        Command::Rng => {
            let rng_states = generate_rng_states(RandOptions::new(
                opts.adiv,
                opts.sdiv,
                opts.adiv_index,
                opts.sdiv_index,
                opts.state,
                opts.start_advance,
                end_advance,
            ));
            for rng_state in rng_states {
                println!(
                    "adv {}, div {:02x}{:02x}, rand {:04x}",
                    rng_state.advance, rng_state.add_div, rng_state.sub_div, rng_state.rand
                );
            }
        }
        Command::Starter => {
            let results = generate_starters(PokeOptions::new(
                opts.adiv,
                opts.sdiv,
                opts.adiv_index,
                opts.sdiv_index,
                opts.state,
                opts.start_advance,
                opts.start_advance + opts.advance_count,
                Filter::Any,
            ));
            println!("{:?}", results);
        }
    };
}
