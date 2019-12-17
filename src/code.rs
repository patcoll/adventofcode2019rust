use std::ops::Range;

use rayon::prelude::*;

/**
 * This function is not efficient, but it is made better by using Rayon parallel filter.
 */
pub fn find_candidates(range: Range<u32>) -> Vec<u32> {
    range
        .clone()
        .into_par_iter()
        .filter(|n| {
            let digits = int_to_digits(*n);
            has_increasing_digits(&digits) && has_adjacent_dup(&digits)
        })
        .collect()
}

fn has_increasing_digits(number: &Vec<u32>) -> bool {
    if number.len() == 1 {
        return false;
    }

    let mut prev: Option<u32> = None;

    number.iter().fold(true, |acc, n| {
        let result = acc
            && match prev {
                Some(p) => *n >= p,
                None => true,
            };
        prev = Some(*n);
        result
    })
}

fn has_adjacent_dup(n: &Vec<u32>) -> bool {
    let mut prev: Option<u32> = None;

    let has_adjacent_dup = n.iter().fold(false, |acc, n| {
        let result = acc
            || match prev {
                Some(p) => *n == p,
                None => false,
            };
        prev = Some(*n);
        result
    });

    has_adjacent_dup
}

fn int_to_digits(input: u32) -> Vec<u32> {
    input
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find() {
        assert_eq!(find_candidates(2..8).len(), 0);
        assert_eq!(find_candidates(2..11).len(), 0);
        assert_eq!(find_candidates(2..12).len(), 1);
        assert_eq!(find_candidates(2..99).len(), 8);
        assert_eq!(find_candidates(2..100).len(), 9);
        assert_eq!(find_candidates(20..99).len(), 7);
        assert_eq!(find_candidates(20..80).len(), 6);
        assert_eq!(find_candidates(200..800).len(), 60);
    }
}
