use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::iter::Iterator;

// const BLACK: u32 = 0;
// const WHITE: u32 = 1;
const TRANSPARENT: u32 = 2;

pub type Grid = Vec<Vec<u32>>;

#[derive(Debug, Default)]
pub struct Image {
    pub width: usize,
    pub height: usize,
    pub layers: Vec<Grid>,
}

impl Image {
    pub fn new(width: usize, height: usize, source: impl Iterator<Item = u32>) -> Self {
        let layers = Self::parse_iterator(width, height, source).unwrap();
        Image {
            width,
            height,
            layers,
        }
    }

    pub fn parse_iterator(
        width: usize,
        height: usize,
        source: impl Iterator<Item = u32>,
    ) -> Result<Vec<Grid>, Box<dyn Error>> {
        let mut digits_vec = vec![];

        for n in source {
            digits_vec.push(n);
        }

        let chunks: Vec<_> = digits_vec.chunks(width).collect();
        let chunks_of_chunks: Vec<_> = chunks.chunks(height).collect();

        let layers = chunks_of_chunks
            .into_par_iter()
            .map(|grid_data| {
                let vectorized_data: Vec<Vec<_>> =
                    grid_data.into_par_iter().map(|d| d.to_vec()).collect();
                vectorized_data
            })
            .collect();

        Ok(layers)
    }

    pub fn count_digits(&self) -> Vec<HashMap<u32, u32>> {
        let mut all_counts: Vec<HashMap<u32, u32>> = vec![];

        for n in self.layers.iter() {
            let mut counts: HashMap<u32, u32> = HashMap::new();

            for i in 0..10 {
                counts.insert(i, 0);
            }

            for j in n.iter().flatten() {
                let val = *(counts.get(&j).get_or_insert(&0)) + 1;
                counts.insert(*j, val);
            }

            all_counts.push(counts);
        }

        all_counts
    }

    pub fn visible(&self) -> Vec<Vec<u32>> {
        let flattened_layers: Vec<Vec<u32>> = self
            .layers
            .clone()
            .into_par_iter()
            .map(|layer| layer.into_iter().flatten().collect())
            .collect();

        let layer_size = flattened_layers[0].len();

        flattened_layers
            .into_iter()
            .fold(
                vec![TRANSPARENT; layer_size],
                |mut memo: Vec<u32>, layer: Vec<u32>| {
                    for (i, n) in layer.into_iter().enumerate() {
                        memo[i] = if memo[i] != 2 { memo[i] } else { n };
                    }
                    memo
                },
            )
            .chunks(self.width)
            .map(|ch| ch.to_owned())
            .collect()
    }

    pub fn print(&self) {
        let visible = self.visible();

        for row in visible {
            for pixel in row {
                let output = if pixel == 1 { '\u{2588}' } else { ' ' };
                print!("{}", output);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::code::Digits;

    #[test]
    fn test_parse_source() {
        assert_eq!(
            Image::parse_iterator(3, 2, Digits::from(123_456_789_012u64)).unwrap(),
            vec![
                (vec![vec![1, 2, 3], vec![4, 5, 6]]),
                (vec![vec![7, 8, 9], vec![0, 1, 2]])
            ]
        );
    }

    #[test]
    fn test_count_digits() {
        let image = Image::new(3, 2, Digits::from(123_456_789_012u64));

        let counts = image.count_digits();

        assert_eq!(counts[0].get(&0).unwrap(), &0);
        assert_eq!(counts[0].get(&1).unwrap(), &1);
        assert_eq!(counts[0].get(&2).unwrap(), &1);
        assert_eq!(counts[0].get(&3).unwrap(), &1);
        assert_eq!(counts[0].get(&4).unwrap(), &1);
        assert_eq!(counts[0].get(&5).unwrap(), &1);
        assert_eq!(counts[0].get(&6).unwrap(), &1);
        assert_eq!(counts[0].get(&7).unwrap(), &0);
        assert_eq!(counts[0].get(&8).unwrap(), &0);
        assert_eq!(counts[0].get(&9).unwrap(), &0);

        assert_eq!(counts[1].get(&0).unwrap(), &1);
        assert_eq!(counts[1].get(&1).unwrap(), &1);
        assert_eq!(counts[1].get(&2).unwrap(), &1);
        assert_eq!(counts[1].get(&3).unwrap(), &0);
        assert_eq!(counts[1].get(&4).unwrap(), &0);
        assert_eq!(counts[1].get(&5).unwrap(), &0);
        assert_eq!(counts[1].get(&6).unwrap(), &0);
        assert_eq!(counts[1].get(&7).unwrap(), &1);
        assert_eq!(counts[1].get(&8).unwrap(), &1);
        assert_eq!(counts[1].get(&9).unwrap(), &1);
    }

    #[test]
    fn test_visible() {
        let input = "0222112222120000"
            .chars()
            .map(|node| node.to_digit(10).unwrap() as u32);

        let image = Image::new(2, 2, input);

        let visible = image.visible();

        assert_eq!(visible, vec![vec![0, 1], vec![1, 0]]);
    }

    #[test]
    fn test_print() {
        let input = "0222112222120000"
            .chars()
            .map(|node| node.to_digit(10).unwrap() as u32);

        let image = Image::new(2, 2, input);

        image.print();
    }
}
