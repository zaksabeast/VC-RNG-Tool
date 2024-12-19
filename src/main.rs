mod div;
mod rng;

use clap::Parser;
use div::Div;
use rng::Rng;
use std::num::ParseIntError;

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
    #[arg(short = 'E', long)]
    end_advance: usize,
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

    let (add_div, sub_div) = Div::new_pair(opts.index, (opts.div >> 8) as u8, opts.div as u8);
    let mut rng = Rng::new(opts.state, add_div, sub_div);

    match opts.command {
        Command::Rng { log_count } => {
            let log_start = opts.end_advance.saturating_sub(log_count);
            for advance in (opts.start_advance + 1)..=opts.end_advance {
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
            for advance in opts.start_advance..=opts.end_advance {
                match rng.is_good_poke() {
                    (true, false) => println!("adv {}, state {:04x}, shiny", advance, rng.state()),
                    (false, true) => println!("adv {}, state {:04x}, max dv", advance, rng.state()),
                    // Not possible to have shiny and max dv
                    _ => {}
                }
                rng.next();
            }
        }
    };
}
