use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::RangeInclusive;

use rayon::prelude::*;

/**
 * This function is not efficient, but it is made better by using Rayon parallel filter.
 */
pub fn find_candidates(range: RangeInclusive<u32>) -> Vec<u32> {
    range.into_par_iter().filter(|n| is_candidate(*n)).collect()
}

fn add_digit(n: &[u32]) -> Vec<u32> {
    let last = &n[n.len() - 1..];
    [n, last].concat()
}

fn int_to_digits2(input: u32) -> Vec<u32> {
    input
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect()
}

fn digits_to_int(input: &[u32]) -> u32 {
    input
        .iter()
        .rev()
        .enumerate()
        .map(|(i, n)| 10u32.pow(i as u32) * *n)
        .sum()
}

fn gen_subsequent_numbers(prefix: &[u32], index: usize) -> Vec<Vec<u32>> {
    let results: Vec<Vec<u32>> = (prefix[index]..=9)
        .map(|n| {
            let mut prefix_clone = prefix.to_vec();
            prefix_clone[index] = n;
            prefix_clone
        })
        .collect();

    results
}

// Iteratively build up range of possible matches using increasing digit heuristic.
pub fn find_candidates2(range: RangeInclusive<u32>) -> Vec<u32> {
    let start = *range.start();
    let end = *range.end();

    // let RangeInclusive { start, end, is_empty: _ } = range;

    let start_string = &start.to_string();
    let start_vec = int_to_digits2(start);
    let start_length = start_string.len();

    let end_string = &end.to_string();
    let end_vec = int_to_digits2(end);
    let end_length = end_string.len();

    let generate = |input: &Vec<Vec<u32>>| -> Vec<Vec<u32>> {
        input
            .into_par_iter()
            .flat_map(|prefix| {
                let added = add_digit(&prefix.to_vec());
                gen_subsequent_numbers(&added, prefix.len())
            })
            .collect()
    };

    // let generate_start_collected = (1..=9).collect::<Vec<_>>();
    // println!("generate_start_collected: {:?}", generate_start_collected);
    let generate_start = if start_length == end_length {
        (start_vec[0]..=end_vec[0])
    } else {
        (1..=9)
    }
    .map(|n| vec![n])
    .collect::<Vec<_>>();
    // println!("generate_start: {:?}", generate_start);

    let lengths = (start_length..=end_length).collect::<HashSet<_>>();

    // println!("lengths: {:?}", lengths);

    if end_length == 1 {
        return vec![];
    }

    let mut all_results: Vec<Vec<u32>> = vec![];
    let mut n_length_result: Vec<Vec<_>> = generate_start;

    loop {
        n_length_result = generate(&n_length_result);
        // println!("n_length_result: {:?}", n_length_result);

        if lengths.contains(&n_length_result[0].len()) {
            all_results.extend(n_length_result.clone());
            // println!("all_results: {:?}", all_results);
        }

        if n_length_result[0].len() == end_length {
            break;
        }
    }

    all_results
        .into_par_iter()
        .map(|prefix| digits_to_int(&prefix))
        .filter(|num| range.contains(num))
        .filter(|&num| has_adjacent_dup(num as u32))
        .collect::<Vec<_>>()
}

pub fn find_candidates_with_one_dup2(range: RangeInclusive<u32>) -> Vec<u32> {
    find_candidates2(range)
        .into_par_iter()
        .filter(|n| is_candidate_with_one_dup(*n))
        .collect()
}

pub fn find_candidates_with_one_dup(range: RangeInclusive<u32>) -> Vec<u32> {
    range
        .into_par_iter()
        .filter(|n| is_candidate_with_one_dup(*n))
        .collect()
}

fn has_increasing_digits(number: u32) -> bool {
    let digits = Digits::from(number);

    if digits.position == 1 {
        return false;
    }

    let mut prev: Option<u32> = None;

    let digits = Digits::from(number);

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

fn is_candidate(n: u32) -> bool {
    has_increasing_digits(n) && has_adjacent_dup(n)
}

#[allow(dead_code)]
fn is_candidate2(n: u32) -> bool {
    find_candidates2(n..=n).len() == 1
}

fn is_candidate_with_one_dup(n: u32) -> bool {
    has_increasing_digits(n) && has_one_adjacent_dup(n)
}

#[allow(dead_code)]
fn is_candidate_with_one_dup2(n: u32) -> bool {
    find_candidates_with_one_dup2(n..=n).len() == 1
}

fn has_adjacent_dup(n: u32) -> bool {
    let mut prev: Option<u32> = None;

    let digits = Digits::from(n);

    digits.fold(false, |acc, n| {
        let result = acc
            || match prev {
                Some(p) => n == p,
                None => false,
            };
        prev = Some(n);
        result
    })
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
fn has_one_adjacent_dup(n: u32) -> bool {
    let digits = Digits::from(n);

    let mut counts: HashMap<u32, u32> = HashMap::new();

    for n in digits {
        let val = *(counts.get(&n).get_or_insert(&0)) + 1;
        counts.insert(n, val);
    }

    counts.into_iter().any(|(_, count)| count == 2)
}

#[derive(Debug, Default)]
pub struct Digits {
    vec: Vec<u32>,
    position: usize,
    rev_position: usize,
}

impl From<u32> for Digits {
    fn from(num: u32) -> Self {
        let vec = Self::number_to_vec(num);
        let length = vec.len();

        Digits {
            vec,
            rev_position: length,
            ..Default::default()
        }
    }
}

impl Digits {
    fn number_to_vec(n: u32) -> Vec<u32> {
        let mut digits = Vec::new();
        let mut n = n;
        while n > 9 {
            digits.push(n % 10);
            n /= 10;
        }
        digits.push(n);
        digits.reverse();
        digits
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.len() == 0
    }
}

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position < self.vec.len() {
            let result = self.vec[self.position];
            self.position += 1;
            Some(result)
        } else {
            None
        }
    }
}

impl DoubleEndedIterator for Digits {
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
        assert_eq!(find_candidates(2..=8).len(), 0);
        assert_eq!(find_candidates(2..=11).len(), 1);
        assert_eq!(find_candidates(2..=12).len(), 1);
        assert_eq!(find_candidates(2..=98).len(), 8);
        assert_eq!(find_candidates(2..=100).len(), 9);
        assert_eq!(find_candidates(20..=99).len(), 8);
        assert_eq!(find_candidates(20..=80).len(), 6);
        assert_eq!(find_candidates(200..=800).len(), 60);
    }

    #[test]
    fn test_find_candidates2() {
        assert_eq!(find_candidates2(2..=8).len(), 0);
        assert_eq!(find_candidates2(2..=11).len(), 1);
        assert_eq!(find_candidates2(2..=12).len(), 1);
        assert_eq!(find_candidates2(2..=98).len(), 8);
        assert_eq!(find_candidates2(2..=100).len(), 9);
        assert_eq!(find_candidates2(20..=99).len(), 8);
        assert_eq!(find_candidates2(20..=80).len(), 6);
        assert_eq!(find_candidates2(200..=800).len(), 60);
    }

    #[test]
    fn test_find_candidates_with_one_dup() {
        assert_eq!(find_candidates_with_one_dup(2..=8).len(), 0);
        assert_eq!(find_candidates_with_one_dup(2..=11).len(), 1);
        assert_eq!(find_candidates_with_one_dup(2..=12).len(), 1);
        assert_eq!(find_candidates_with_one_dup(2..=98).len(), 8);
        assert_eq!(find_candidates_with_one_dup(2..=100).len(), 9);
        assert_eq!(find_candidates_with_one_dup(20..=98).len(), 7);
        assert_eq!(find_candidates_with_one_dup(20..=80).len(), 6);
        assert_eq!(find_candidates_with_one_dup(200..=800).len(), 54);
    }

    #[test]
    fn test_find_candidates_with_one_dup2() {
        assert_eq!(find_candidates_with_one_dup2(2..=8).len(), 0);
        assert_eq!(find_candidates_with_one_dup2(2..=11).len(), 1);
        assert_eq!(find_candidates_with_one_dup2(2..=12).len(), 1);
        assert_eq!(find_candidates_with_one_dup2(2..=99).len(), 9);
        assert_eq!(find_candidates_with_one_dup2(2..=100).len(), 9);
        assert_eq!(find_candidates_with_one_dup2(20..=99).len(), 8);
        assert_eq!(find_candidates_with_one_dup2(20..=80).len(), 6);
        assert_eq!(find_candidates_with_one_dup2(200..=800).len(), 54);
    }

    #[test]
    fn test_is_candidate() {
        assert_eq!(is_candidate(22), true);
        assert_eq!(is_candidate(222), true);
        assert_eq!(is_candidate(111_111), true);
        assert_eq!(is_candidate(223_450), false);
        assert_eq!(is_candidate(123_789), false);
    }

    #[test]
    fn test_is_candidate2() {
        assert_eq!(is_candidate2(22), true);
        assert_eq!(is_candidate2(222), true);
        assert_eq!(is_candidate2(111_111), true);
        assert_eq!(is_candidate2(223_450), false);
        assert_eq!(is_candidate2(123_789), false);
    }

    #[test]
    fn test_is_candidate_with_one_dup() {
        assert_eq!(is_candidate_with_one_dup(112_233), true);
        assert_eq!(is_candidate_with_one_dup(112_344), true);
        assert_eq!(is_candidate_with_one_dup(113_444), true);
        assert_eq!(is_candidate_with_one_dup(124_444), false);
        assert_eq!(is_candidate_with_one_dup(123_444), false);
        assert_eq!(is_candidate_with_one_dup(111_122), true);
        assert_eq!(is_candidate_with_one_dup(112_222), true);
        assert_eq!(is_candidate_with_one_dup(223_334), true);
    }

    #[test]
    fn test_is_candidate_with_one_dup2() {
        assert_eq!(is_candidate_with_one_dup2(112_233), true);
        assert_eq!(is_candidate_with_one_dup2(112_344), true);
        assert_eq!(is_candidate_with_one_dup2(113_444), true);
        assert_eq!(is_candidate_with_one_dup2(124_444), false);
        assert_eq!(is_candidate_with_one_dup2(123_444), false);
        assert_eq!(is_candidate_with_one_dup2(111_122), true);
        assert_eq!(is_candidate_with_one_dup2(112_222), true);
        assert_eq!(is_candidate_with_one_dup2(223_334), true);
    }
}
