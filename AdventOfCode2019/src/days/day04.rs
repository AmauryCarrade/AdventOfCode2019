use crate::{first_answer, second_answer};

use itertools::Itertools;
use std::ops::Range;

const RANGE: Range<u32> = 256_310..732_737;

pub fn run() {
    first_answer(
        "Number of possible passwords (mathematical solution)",
        &RANGE
            .filter(ordered_numbers)
            .filter(|i| number_groups_sizes_match(i, |n| n >= 2))
            .count(),
    );

    second_answer(
        "Number of possible passwords with the additional rule",
        &RANGE
            .filter(ordered_numbers)
            .filter(|i| number_groups_sizes_match(i, |n| n == 2))
            .count(),
    );
}

///
/// Checks (the mathematical way) if all digits of the 6-digits given number
/// are ordered (e.g. 111111 passes, 124578 too, but 124794 don't).
///
/// ```rust
/// # use crate::lib::days::day04::ordered_numbers;
/// assert!(ordered_numbers(&111111));
/// assert!(ordered_numbers(&124578));
/// assert!(!ordered_numbers(&124794));
/// ```
pub fn ordered_numbers(i: &u32) -> bool {
    (0..6)
        .rev()
        .fold((0, true), |(prev, is_ok), digit_pos| {
            (
                (i / 10_u32.pow(digit_pos)) % 10,
                is_ok & ((i / 10_u32.pow(digit_pos)) % 10 >= prev),
            )
        })
        .1
}

///
/// Checks if all sizes of groups of identical adjacent digits
/// meet the given criteria.
///
/// ```rust
/// # use crate::lib::days::day04::number_groups_sizes_match;
/// assert!(number_groups_sizes_match(&111_111, |size| size >= 2));
/// assert!(!number_groups_sizes_match(&123_789, |size| size >= 2));
/// assert!(!number_groups_sizes_match(&111_111, |size| size == 2));
/// ```
pub fn number_groups_sizes_match<F>(i: &u32, predicate: F) -> bool
where
    F: Fn(u8) -> bool,
{
    i.to_string()
        .chars()
        .group_by(|d| d.clone())
        .into_iter()
        .any(|(_group, iter)| predicate(iter.count() as u8))
}
