#![deny(clippy::all)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]
#![feature(num_midpoint)]
#![feature(iter_next_chunk)]
#![feature(iter_array_chunks)]
mod day05;
fn main() {
    day05::part1();
    day05::part1_take2();
    day05::part2();
}
