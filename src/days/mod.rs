mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub use day1::day1;
pub use day2::day2;
pub use day3::day3;
pub use day4::day4;
pub use day5::day5;
pub use day6::day6;
pub use day7::day7;
pub use day8::day8;
pub use day9::day9;
use num_traits::PrimInt;

fn parse_integer<I>(s: &str) -> I
where
    I: PrimInt,
{
    s.as_bytes().iter().fold(I::zero(), |a, n| {
        a.mul(I::from(10).unwrap())
            .add(I::from(*n).unwrap())
            .sub(I::from(48).unwrap())
    })
}
