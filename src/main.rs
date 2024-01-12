use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long="rule", default_value_t = 30)]
    ///rule (0..255) 
    r: u8,

    #[arg(short, long="state", default_value_t = String::from("0x80000000"))]
    ///input (u64) - use binary (0b), octal (0o), hex (0x) or decimal notation
    s: String,
}

fn apply_rule(state: u64, rule: u8) -> u64 {
    let mut new_state = 0;

    for i in 0..64 {
        let left_neighbor = (state >> ((i + 63) % 64) & 1) as u8;
        let center = (state >> i & 1) as u8;
        let right_neighbor = (state >> ((i + 1) % 64) & 1) as u8;

        let neighbors = match i {
            0 => (center << 1) | right_neighbor,
            63  => (left_neighbor << 2) | (center << 1),
            _ => (left_neighbor << 2) | (center << 1) | right_neighbor,
        };
       
        let new_bit = (rule >> neighbors) & 1;
        new_state |= (new_bit as u64) << i;
    }

    new_state
}

fn main() {
    let args = Args::parse();

    let s = if args.s.starts_with("0b") {
        u64::from_str_radix(&args.s[2..], 2)
    } else if args.s.starts_with("0o") {
        u64::from_str_radix(&args.s[2..], 8)
    } else if args.s.starts_with("0x") {
        u64::from_str_radix(&args.s[2..], 16)
    } else {
        u64::from_str_radix(&args.s, 10)
    };

    //let mut state = 1u64 << 31;
    let mut state = s.expect("Failed to parse input");


    for _ in 0..32 {
        for j in 0..64 {
            print!("{}", if state >> j & 1 == 1 { 'O' } else { '.' });
        }
        println!();
        //state = apply_rule(state, args.r);
        //state = (state >> 1) ^ (state | state << 1); // rule 30
        //state = (state >> 1) ^ (state << 1); // rule 90
    }
}

