//! 1D Cellular Automata - minimal dependencies, fixed width of 64
//! Generates ascii output

use clap::Parser;
use std::{io, io::Write};

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Elementary Cellular Automaton (ECA) Simulator"
)]
struct Args {
    #[arg(short, long, default_value_t = 30)]
    ///rule (0..255)
    rule: u8,

    #[arg(short, long, default_value = "0x80000000", value_parser = parse_prefixed_u64)]
    /// Input (u64) - use binary (0b), octal (0o), hex (0x) or decimal notation
    state: u64,

    #[arg(short = 'S', long, default_value_t = 32)]
    ///number of steps (0..)
    steps: usize,
}

/// Custom parser for clap that handles Rust-style integer literals
fn parse_prefixed_u64(s: &str) -> Result<u64, String> {
    let (input, radix) = if let Some(stripped) = s.strip_prefix("0b") {
        (stripped, 2)
    } else if let Some(stripped) = s.strip_prefix("0x") {
        (stripped, 16)
    } else if let Some(stripped) = s.strip_prefix("0o") {
        (stripped, 8)
    } else {
        (s, 10)
    };

    u64::from_str_radix(input.replace('_', "").as_str(), radix)
        .map_err(|e| format!("Invalid number '{s}': {e}"))
}

/// Applies the ECA rule using periodic (wrapping) boundary conditions
fn apply_rule(state: u64, rule: u8) -> u64 {
    let mut new_state = 0u64;

    for i in 0..64 {
        // Use wrapping logic for periodic boundaries
        let left = (state >> ((i + 1) % 64)) & 1;
        let center = (state >> i) & 1;
        let right = (state >> ((i + 63) % 64)) & 1;

        // Combine neighbors into a 3-bit index (0-7)
        let pattern = (left << 2) | (center << 1) | right;

        // Check if the bit at that index in the rule is 1
        if (rule >> pattern) & 1 == 1 {
            new_state |= 1 << i;
        }
    }

    new_state
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut state = args.state;

    // faster terminal output
    let mut stdout = io::BufWriter::new(io::stdout());

    for _ in 0..args.steps {
        for i in (0..64).rev() {
            let symbol = if (state >> i) & 1 == 1 { 'O' } else { '.' };
            write!(stdout, "{symbol}")?;
        }
        writeln!(stdout)?;
        state = match args.rule {
            30 => (state >> 1) ^ (state | state << 1), // fast path
            90 => (state >> 1) ^ (state << 1),         // fast path
            _ => apply_rule(state, args.rule),
        };
    }

    stdout.flush()?;
    Ok(())
}
