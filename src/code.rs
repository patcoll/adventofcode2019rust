use std::ops::Range;
use std::collections::HashMap;
use std::collections::HashSet;

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

fn add_digit(n: &Vec<u32>) -> Vec<u32> {
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

fn digits_to_int(input: &Vec<u32>) -> u32 {
    input
        .clone()
        .iter()
        .rev()
        .enumerate()
        .map(|(i, n)| 10u32.pow(i as u32) * *n)
        .sum()
}

fn gen_subsequent_numbers(prefix: &Vec<u32>, index: usize) -> Vec<Vec<u32>> {
    let results: Vec<Vec<u32>> = (prefix[index]..=9)
        .map(|n| {
            let mut prefix_clone = prefix.clone();
            prefix_clone[index] = n;
            prefix_clone
        })
        .collect();

    results
}

// Iteratively build up range of possible matches using increasing digit heuristic.
pub fn find_candidates2(range: Range<u32>) -> Vec<u32> {
    let Range { start, end } = range;

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
                let generated = gen_subsequent_numbers(&added, prefix.len());
                generated
            })
            .collect()
    };

    // let generate_start_collected = (1..=9).collect::<Vec<_>>();
    // println!("generate_start_collected: {:?}", generate_start_collected);
    let generate_start =
        if start_length == end_length {
            (start_vec[0]..=end_vec[0])
        } else {
            (1..=9)
        }.map(|n| vec![n]).collect::<Vec<_>>();
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

    let results = all_results
        .into_par_iter()
        .map(|prefix| digits_to_int(&prefix))
        .filter(|num| num >= &start && num < &end)
        .filter(|num| has_adjacent_dup(&(*num as u32)))
        .collect::<Vec<_>>();

    // println!("results: {:?}", results);
    results
}

pub fn find_candidates_with_one_dup2(range: Range<u32>) -> Vec<u32> {
    find_candidates2(range)
        .into_par_iter()
        .filter(|n| is_candidate_with_one_dup(n))
        .collect()
}

pub fn find_candidates_with_one_dup(range: Range<u32>) -> Vec<u32> {
    range
        .into_par_iter()
        .filter(|n| is_candidate_with_one_dup(n))
        .collect()
}

fn has_increasing_digits(number: &u32) -> bool {
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

fn is_candidate(n: &u32) -> bool {
    has_increasing_digits(n) && has_adjacent_dup(n)
}

fn is_candidate2(n: &u32) -> bool {
    find_candidates2(*n..*n + 1).len() == 1
}

fn is_candidate_with_one_dup(n: &u32) -> bool {
    has_increasing_digits(n) && has_one_adjacent_dup(n)
}

fn is_candidate_with_one_dup2(n: &u32) -> bool {
    find_candidates_with_one_dup2(*n..*n + 1).len() == 1
}

fn has_adjacent_dup(n: &u32) -> bool {
    let mut prev: Option<u32> = None;

    let digits = Digits::from(n);

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
    let digits = Digits::from(n);

    let mut counts: HashMap<u32, u32> = HashMap::new();

    for n in digits {
        let val = *(counts.get(&n).get_or_insert(&0)) + 1;
        counts.insert(n, val);
    }

    counts
        .into_iter()
        .any(|(_, count)| count == 2)
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
        let vec = Self::number_to_vec(*num);
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
            n = n / 10;
        }
        digits.push(n);
        digits.reverse();
        digits
    }


    pub fn len(&self) -> usize {
        self.vec.len()
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
    fn test_find_candidates2() {
        assert_eq!(find_candidates2(2..8).len(), 0);
        assert_eq!(find_candidates2(2..11).len(), 0);
        assert_eq!(find_candidates2(2..12).len(), 1);
        assert_eq!(find_candidates2(2..99).len(), 8);
        assert_eq!(find_candidates2(2..100).len(), 9);
        assert_eq!(find_candidates2(20..99).len(), 7);
        assert_eq!(find_candidates2(20..80).len(), 6);
        assert_eq!(find_candidates2(200..800).len(), 60);
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
    fn test_find_candidates_with_one_dup2() {
        assert_eq!(find_candidates_with_one_dup2(2..8).len(), 0);
        assert_eq!(find_candidates_with_one_dup2(2..11).len(), 0);
        assert_eq!(find_candidates_with_one_dup2(2..12).len(), 1);
        assert_eq!(find_candidates_with_one_dup2(2..99).len(), 8);
        assert_eq!(find_candidates_with_one_dup2(2..100).len(), 9);
        assert_eq!(find_candidates_with_one_dup2(20..99).len(), 7);
        assert_eq!(find_candidates_with_one_dup2(20..80).len(), 6);
        assert_eq!(find_candidates_with_one_dup2(200..800).len(), 54);
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
    fn test_is_candidate2() {
        assert_eq!(is_candidate2(&22), true);
        assert_eq!(is_candidate2(&222), true);
        assert_eq!(is_candidate2(&111111), true);
        assert_eq!(is_candidate2(&223450), false);
        assert_eq!(is_candidate2(&123789), false);
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

    #[test]
    fn test_is_candidate_with_one_dup2() {
        assert_eq!(is_candidate_with_one_dup2(&112233), true);
        assert_eq!(is_candidate_with_one_dup2(&112344), true);
        assert_eq!(is_candidate_with_one_dup2(&113444), true);

        assert_eq!(is_candidate_with_one_dup2(&124444), false);

        assert_eq!(is_candidate_with_one_dup2(&123444), false);
        assert_eq!(is_candidate_with_one_dup2(&111122), true);
        assert_eq!(is_candidate_with_one_dup2(&112222), true);
        assert_eq!(is_candidate_with_one_dup2(&223334), true);
    }
}
