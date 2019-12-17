use std::ops::Range;
use std::collections::HashMap;

use rayon::prelude::*;

/**
 * This function is not efficient, but it is made better by using Rayon parallel filter.
 */
pub fn find_candidates(range: Range<u32>) -> Vec<u32> {
    range
        .into_par_iter()
        .filter(|n| is_candidate(n))
        .collect()
}

pub fn find_candidates_with_one_dup(range: Range<u32>) -> Vec<u32> {
    range
        .into_par_iter()
        .filter(|n| is_candidate_with_one_dup(n))
        .collect()
}

fn has_increasing_digits(number: &u32) -> bool {
    let digits = int_to_digits(number);

    if digits.position == 1 {
        return false;
    }

    let mut prev: Option<u32> = None;

    let digits = int_to_digits(number);

    digits.fold(true, |acc, n| {
        let result = acc
            && match prev {
                Some(p) => n >= p,
                None => true,
            };
        prev = Some(n);
        result
    })
}

fn is_candidate(n: &u32) -> bool {
    has_increasing_digits(n) && has_adjacent_dup(n)
}

fn is_candidate_with_one_dup(n: &u32) -> bool {
    has_increasing_digits(n) && has_one_adjacent_dup(n)
}

fn has_adjacent_dup(n: &u32) -> bool {
    let mut prev: Option<u32> = None;

    let digits = int_to_digits(n);

    let has_adjacent_dup = digits.fold(false, |acc, n| {
        let result = acc
            || match prev {
                Some(p) => n == p,
                None => false,
            };
        prev = Some(n);
        result
    });

    has_adjacent_dup
}

/**
 * The wording is confusing, but found this clarification (from
 * https://dev.to/jbristow/advent-of-code-2019-solution-megathread-day-4-secure-container-255c):
 *
 * > there needs to be at least one number that appears exactly twice
 *
 * Also see:
 * https://www.reddit.com/r/adventofcode/comments/e5uatc/the_two_adjacent_matching_digits_are_not_part_of/
 */
fn has_one_adjacent_dup(n: &u32) -> bool {
    let digits = int_to_digits(n);

    let mut counts: HashMap<u32, u32> = HashMap::new();

    for n in digits {
        let val = *(counts.get(&n).get_or_insert(&0)) + 1;
        counts.insert(n, val);
    }

    counts
        .into_iter()
        .any(|(_, count)| count == 2)
}

fn int_to_digits(input: &u32) -> Digits {
    let vec = number_to_vec(*input);
    let length = vec.len();

    Digits {
        vec: number_to_vec(*input),
        position: 0,
        rev_position: length,
    }
}

fn number_to_vec(n: u32) -> Vec<u32> {
    let mut digits = Vec::new();
    let mut n = n;
    while n > 9 {
        digits.push(n % 10);
        n = n / 10;
    }
    digits.push(n);
    digits.reverse();
    digits
}










#[derive(Debug, Default)]
pub struct Digits {
    vec: Vec<u32>,
    position: usize,
    rev_position: usize,
}

impl From<u32> for Digits {
    fn from(num: u32) -> Self {
        Digits::from(&num)
    }
}

impl From<&u32> for Digits {
    fn from(num: &u32) -> Self {
        int_to_digits(num)
    }
}

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.vec.len() {
            // println!("self.num: {:?}", self.num);
            // println!("self.position: {:?}", self.position);
            let result = self.vec[self.position];
            self.position += 1;
            // self.num %= self.position;
            // println!("self.num: {:?}", self.num);
            // self.position /= 10;
            // println!("result: {:?}", result);
            // println!("");
            Some(result)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for Digits {
    // type Item = u32;

    fn next_back(&mut self) -> Option<Self::Item> {
        if self.rev_position > 0 {
            let result = self.vec[self.rev_position - 1];
            self.rev_position -= 1;
            Some(result)
        } else {
            None
        }
    }
}











#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_candidates() {
        assert_eq!(find_candidates(2..8).len(), 0);
        assert_eq!(find_candidates(2..11).len(), 0);
        assert_eq!(find_candidates(2..12).len(), 1);
        assert_eq!(find_candidates(2..99).len(), 8);
        assert_eq!(find_candidates(2..100).len(), 9);
        assert_eq!(find_candidates(20..99).len(), 7);
        assert_eq!(find_candidates(20..80).len(), 6);
        assert_eq!(find_candidates(200..800).len(), 60);
    }

    #[test]
    fn test_find_candidates_with_one_dup() {
        assert_eq!(find_candidates_with_one_dup(2..8).len(), 0);
        assert_eq!(find_candidates_with_one_dup(2..11).len(), 0);
        assert_eq!(find_candidates_with_one_dup(2..12).len(), 1);
        assert_eq!(find_candidates_with_one_dup(2..99).len(), 8);
        assert_eq!(find_candidates_with_one_dup(2..100).len(), 9);
        assert_eq!(find_candidates_with_one_dup(20..99).len(), 7);
        assert_eq!(find_candidates_with_one_dup(20..80).len(), 6);
        assert_eq!(find_candidates_with_one_dup(200..800).len(), 54);
    }

    #[test]
    fn test_is_candidate() {
        assert_eq!(is_candidate(&22), true);
        assert_eq!(is_candidate(&222), true);
        assert_eq!(is_candidate(&111111), true);
        assert_eq!(is_candidate(&223450), false);
        assert_eq!(is_candidate(&123789), false);
    }

    #[test]
    fn test_is_candidate_with_one_dup() {
        assert_eq!(is_candidate_with_one_dup(&112233), true);
        assert_eq!(is_candidate_with_one_dup(&112344), true);
        assert_eq!(is_candidate_with_one_dup(&113444), true);

        assert_eq!(is_candidate_with_one_dup(&124444), false);

        assert_eq!(is_candidate_with_one_dup(&123444), false);
        assert_eq!(is_candidate_with_one_dup(&111122), true);
        assert_eq!(is_candidate_with_one_dup(&112222), true);
        assert_eq!(is_candidate_with_one_dup(&223334), true);
    }
}
