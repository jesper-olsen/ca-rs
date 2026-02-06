use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Elementary Cellular Automaton Simulator")]
struct Args {
    #[arg(short, long, default_value_t = 30)]
    ///rule (0..255)
    rule: u8,

    #[arg(short, long, default_value = "0x80000000", value_parser = parse_prefixed_u64)]
    /// Input (u64) - use binary (0b), octal (0o), hex (0x) or decimal notation
    state: u64,

    #[arg(short = 'S', long, default_value_t = 32)]
    ///number of steps (0..)
    steps: u8,
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

fn apply_rule(state: u64, rule: u8) -> u64 {
    let mut new_state = 0;

    for i in 0..64 {
        let left_neighbor = (state >> ((i + 63) % 64) & 1) as u8;
        let center = (state >> i & 1) as u8;
        let right_neighbor = (state >> ((i + 1) % 64) & 1) as u8;

        let neighbors = match i {
            0 => (center << 1) | right_neighbor,
            63 => (left_neighbor << 2) | (center << 1),
            _ => (left_neighbor << 2) | (center << 1) | right_neighbor,
        };

        let new_bit = (rule >> neighbors) & 1;
        new_state |= (new_bit as u64) << i;
    }

    new_state
}

#[inline]
fn match_bit(val: u64, bit_expected: u8) -> u64 {
    if bit_expected == 1 { val } else { !val }
}

fn main() {
    let args = Args::parse();

    //let mut state = 1u64 << 31;
    let mut state = args.state;

    for _ in 0..args.steps {
        let row: String = (0..64)
            .map(|i| if (state >> i) & 1 == 1 { 'O' } else { '.' })
            .collect();
        println!("{row}");
        state = apply_rule(state, args.rule);
        //state = (state >> 1) ^ (state | state << 1); // rule 30
        //state = (state >> 1) ^ (state << 1); // rule 90
    }
}
