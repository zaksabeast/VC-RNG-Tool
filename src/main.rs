mod div;
mod rng;

use clap::Parser;
use div::Div;
use rng::Rng;
use std::num::ParseIntError;

fn parse_u8_hex(input: &str) -> Result<u8, ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn parse_u16_hex(input: &str) -> Result<u16, ParseIntError> {
    u16::from_str_radix(input, 16)
}

fn parse_usize_hex(input: &str) -> Result<usize, ParseIntError> {
    usize::from_str_radix(input, 16)
}

fn limit_index_range(input: &str) -> Result<usize, String> {
    let input = usize::from_str_radix(input, 10).map_err(|e| e.to_string())?;
    if input < 9 || input > 15680 {
        Err("Index must be between 9 and 15680".to_string())
    } else {
        Ok(input)
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
/// Reference 3ds VC Rng Tool.
/// Currently only works with Crystal.
struct Cli {
    #[arg(short, long, value_parser = parse_u8_hex)]
    div: u8,
    #[arg(short, long, value_parser = limit_index_range)]
    index: usize,
    #[arg(short, long, value_delimiter = ',', value_parser = parse_usize_hex)]
    adjustments: Vec<usize>,
    #[arg(short, long, value_parser = parse_u16_hex)]
    state: u16,
    #[arg(short = 'S', long)]
    start_advance: usize,
    #[arg(short = 'E', long)]
    end_advance: usize,
    #[arg(short, long)]
    log_count: usize,
}

fn main() {
    let opts = Cli::parse();

    let sub_index = opts.index.wrapping_sub(9);
    let sub_adjustments = opts
        .adjustments
        .iter()
        .map(|&adjustment| adjustment.wrapping_add(0x2c0))
        .collect();

    let sub_div = Div::new(sub_index, opts.div, sub_adjustments);
    let add_div = Div::new(opts.index, opts.div, opts.adjustments);

    let mut rng = Rng::new(opts.state, add_div, sub_div);

    let log_start = opts.end_advance.saturating_sub(opts.log_count);
    for advance in opts.start_advance..opts.end_advance {
        let rand = rng.next_u16();
        if advance >= log_start {
            println!(
                "adv {}, add_div {:02x}, rand {:04x}",
                advance + 1,
                rng.div(),
                rand
            );
        }
    }
}
