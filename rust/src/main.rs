#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![feature(num_midpoint)]
#![feature(iter_next_chunk)]
#![feature(iter_array_chunks)]
mod day03;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    day03::part1()?;
    day03::part2();
    Ok(())
}
