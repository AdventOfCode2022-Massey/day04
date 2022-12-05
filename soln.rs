// This program is licensed under the "MIT License".
// Please see the file LICENSE in this distribution
// for license terms.

//! Advent of Code Day 4.  
//! Ben and Bart Massey 2022

use std::ops::RangeInclusive;

use aoc::{*, reparse::*};

/// Extension trait for comparing ranges with ranges.
trait RangeInclusiveExt<Idx> {
    /// True iff this range contains the `other` range.
    fn contains_range(&self, other: &RangeInclusive<Idx>) -> bool;
    /// True iff the tail of thhis range contains the head
    /// of the `other` range.
    fn laps_range(&self, other: &RangeInclusive<Idx>) -> bool;
}

impl<Idx> RangeInclusiveExt<Idx> for RangeInclusive<Idx>
where
    Idx: PartialOrd,
{
    fn contains_range(&self, other: &RangeInclusive<Idx>) -> bool {
        self.start() >= other.start() && self.end() <= other.end()
    }

    fn laps_range(&self, other: &RangeInclusive<Idx>) -> bool {
        self.contains(other.start())
    }
}

fn main() {
    // Parse the input into inclusive ranges.
    let parser = Reparse::new(r"([0-9]+)-([0-9]+),([0-9]+)-([0-9]+)");
    let input = input_lines().map(|line| {
        let parsed = parser.parse(&line).unwrap();
        let result: (RangeInclusive<u64>, RangeInclusive<u64>) = (
            parsed.get(1)..=parsed.get(2),
            parsed.get(3)..=parsed.get(4),
        );
        result
    });

    match get_part() {
        Part1 => {
            let total = input
                .filter(|(r1, r2)| {
                    r1.contains_range(r2) || r2.contains_range(r1)
                })
                .count();
            println!("{total}");
        }
        Part2 => {
            let total = input
                .filter(|(r1, r2)| {
                    r1.laps_range(r2) || r2.laps_range(r1)
                })
                .count();
            println!("{total}");
        }
    }
}
