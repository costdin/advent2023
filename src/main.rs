mod days;

use days::*;
use std::time::Instant;

fn main() {
    let s1 = Instant::now();
    day1();
    println!("The time is probably {}µs", s1.elapsed().as_micros());
}
