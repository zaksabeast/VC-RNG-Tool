use clap::Parser;
use std::num::ParseIntError;
use vc_rng_lib::{Filter, Options, generate_starters, rng::Rng};

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
    div: u16,
    #[arg(short, long, value_parser = limit_index_range)]
    index: usize,
    #[arg(short, long, value_parser = parse_u16_hex)]
    state: u16,
    #[arg(short = 'S', long)]
    start_advance: usize,
    #[arg(short = 'c', long)]
    advance_count: usize,
}

#[derive(Parser)]
enum Command {
    Rng {
        #[arg(short, long)]
        log_count: usize,
    },
    Starter,
}

fn main() {
    let opts = Cli::parse();
    let end_advance = opts.start_advance + opts.advance_count;

    match opts.command {
        Command::Rng { log_count } => {
            let mut rng = Rng::new_from_div(opts.index, opts.state, opts.div);
            let log_start = end_advance.saturating_sub(log_count);
            for advance in (opts.start_advance + 1)..=end_advance {
                let rand = rng.next_u16();
                if advance >= log_start {
                    println!(
                        "adv {}, div {:02x}{:02x}, rand {:04x}",
                        advance,
                        rng.adiv(),
                        rng.sdiv(),
                        rand
                    );
                }
            }
        }
        Command::Starter => {
            let results = generate_starters(Options::new(
                opts.div,
                opts.index,
                opts.state,
                opts.start_advance,
                opts.start_advance + opts.advance_count,
                Filter::Any,
            ));
            println!("{:?}", results);
        }
    };
}
