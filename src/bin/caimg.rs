use clap::Parser;
use image::{ImageBuffer, Luma};
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about = "ECA to PNG Generator")]
struct Args {
    #[arg(short, long, default_value_t = 30)]
    rule: u8,

    #[arg(short, long, default_value_t = 1000)]
    width: u32,

    #[arg(short, long, default_value_t = 500)]
    steps: u32,

    #[arg(short, long, default_value = "output.png")]
    output: String,

    #[arg(short, long)]
    /// Initial state as string of '0' and '1'
    initial: Option<String>,
}

fn apply_rule(current: &[u8], rule: u8) -> Vec<u8> {
    let len = current.len();
    let mut next = vec![0; len];

    for i in 0..len {
        let left = current[(i + len - 1) % len];
        let center = current[i];
        let right = current[(i + 1) % len];

        let pattern = (left << 2) | (center << 1) | right;

        if (rule >> pattern) & 1 == 1 {
            next[i] = 1;
        }
    }
    next
}

fn main() {
    let args = Args::parse();
    let steps = args.steps as usize;
    let width = args.width as usize;

    // Initialize state: use provided string or a single '1' in the center
    let mut state = vec![0u8; width];
    if let Some(s) = args.initial {
        state
            .iter_mut()
            .zip(s.chars())
            .for_each(|(s, c)| *s = if c == '1' { 1 } else { 0 });
    } else if args.width > 0 {
        state[width / 2] = 1;
    }

    let mut img_buffer = ImageBuffer::new(width as u32, steps as u32);
    for y in 0..steps {
        for (x, &cell) in state.iter().enumerate() {
            const BLACK: u8 = 0;
            const WHITE: u8 = 255;
            let pixel_val = if cell == 1 { WHITE } else { BLACK };
            img_buffer.put_pixel(x as u32, y as u32, Luma([pixel_val])); // use Luma<u8> for grayscale
        }
        state = apply_rule(&state, args.rule);
    }

    img_buffer
        .save(Path::new(&args.output))
        .expect("Failed to save image");
    println!("Saved simulation to {}", args.output);
}
