//! 1D Cellular Automata - generates PNG images - arbitrary width/height

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

fn apply_rule(current: &[u8], next: &mut [u8], rule: u8) {
    let len = current.len();
    for i in 0..len {
        let left = current[(i + len - 1) % len];
        let center = current[i];
        let right = current[(i + 1) % len];
        let pattern = (left << 2) | (center << 1) | right;
        next[i] = (rule >> pattern) & 1;
    }
}

fn parse_initial(s: &str, width: usize) -> Vec<u8> {
    s.chars()
        .take(width)
        .map(|c| if c == '1' { 1 } else { 0 })
        .chain(std::iter::repeat(0))
        .take(width)
        .collect()
}

fn main() {
    let args = Args::parse();
    let steps = args.steps as usize;
    let width = args.width as usize;

    // Initialize state: use provided string or a single '1' in the center
    let mut state: Vec<u8> = args
        .initial
        .as_ref()
        .map(|s| parse_initial(s, width))
        .unwrap_or_else(|| {
            let mut v = vec![0u8; width];
            if width > 0 {
                v[width / 2] = 1;
            }
            v
        });
    let mut next_state = vec![0u8; width];

    let mut img_buffer = ImageBuffer::new(width as u32, steps as u32);

    for y in 0..steps {
        for (x, &cell) in state.iter().enumerate() {
            // use Luma<u8> for grayscale
            const BLACK: u8 = 0;
            const WHITE: u8 = 255;
            let pixel_val = if cell == 1 { WHITE } else { BLACK };
            img_buffer.put_pixel(x as u32, y as u32, Luma([pixel_val])); 
        }
        apply_rule(&state, &mut next_state, args.rule);
        std::mem::swap(&mut state, &mut next_state);
    }

    img_buffer
        .save(Path::new(&args.output))
        .expect("Failed to save image");
    println!("Saved simulation to {}", args.output);
}
