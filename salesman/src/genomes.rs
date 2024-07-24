use rand::{distributions::WeightedIndex, prelude::*};
use std::{collections::HashSet};

use crate::map;


pub fn generate<R: Rng>(size: u8, rng: &mut R) -> Vec<u8> {
    let mut genome = (0..size as u8).collect::<Vec<u8>>();
    genome.shuffle(rng);
    genome
}

pub fn fitness(genome: &[u8], map: &map::Map) -> f32 {
    let length = genome.len();
    1.0 / (
        genome
            .windows(2)
            .map(|pair| distance(
                &map.coords(pair[0] as usize),
                &map.coords(pair[1] as usize),
            ))
            .sum::<f32>() + distance(
                &map.coords(genome[length-1] as usize),
                &map.coords(genome[0] as usize),
            )
    )
}

fn gen_crossover_range(length: usize, rng: &mut impl Rng) -> (usize, usize) {
    let start = rng.gen::<usize>() % (length - 1);
    let remaining_width = length - start;
    let width = 2 + ((rng.gen::<usize>() % (remaining_width+1)).saturating_sub(2));
    (start, start+width)
}

pub fn crossover<R: Rng>(a: &[u8], b: &[u8], rng: &mut R) -> Vec<u8> {
    let size = a.len();
    let (start, end) = gen_crossover_range(size, rng);
    println!("{}, {}", start, end);

    let mut values: HashSet<u8> = HashSet::with_capacity(a.len());
    values.extend(&a[start..end]);

    let mut b_index: usize = 0;
    let mut result: Vec<u8> = Vec::with_capacity(size);
    for i in 0..size {
        if start <= i && i < end {
            result.push(a[i]);
        } else {
            while values.contains(&b[b_index]) {
                b_index += 1;
            }

            result.push(b[b_index]);
            values.insert(b[b_index]);
        }
    }

    return result;
}

pub fn mutate<R: Rng>(genome: &mut [u8], rng: &mut R) {
    let size = genome.len();
    let a = rng.gen::<usize>() % size;
    let mut b;
    while {
        b = rng.gen::<usize>() % size;
        b == a
    }{}
    genome.swap(a, b);
}

pub fn select<R: Rng>(population: &[u8], fitnesses: &[f32], rng: &mut R) -> Vec<u8> {
    let genome_size = population.len() / fitnesses.len();
    let dist = WeightedIndex::new(fitnesses).unwrap();

    let first_index = dist.sample(rng) * genome_size;
    let first = &population[first_index..first_index + genome_size];

    let second_index = loop {
        let index = dist.sample(rng) * genome_size;
        if index != first_index {
            break index;
        }
    };
    let second = &population[second_index..second_index + genome_size];

    let mut result = first.to_vec();
    result.extend_from_slice(second);

    return result;
}

fn distance(a: &[u8; 2], b: &[u8; 2]) -> f32 {
    let x = (i16::from(b[0]) - i16::from(a[0])).abs() as u32;
    let y = (i16::from(b[1]) - i16::from(a[1])).abs() as u32;
    ((x * x + y * y) as f32).sqrt()
}

#[cfg(test)]
mod tests {
    use rand::rngs::mock::StepRng;

    use super::*;

    #[test]
    fn test_select() {
        let population = vec![
            1,  2,  3,  4,  5,
            6,  7,  8,  9,  10,
            11, 12, 13, 14, 15,
            16, 17, 18, 19, 20,
            21, 22, 23, 24, 25,
        ];
        let fitnesses: Vec<f32> = vec![0.0, 1.0, 0.0, 0.0, 1.0];
        let mut rng = StepRng::new(0, 1000);
        
        let result = select(&population, &fitnesses, &mut rng);

        assert_eq!(result, vec![6, 7, 8, 9, 10, 21, 22, 23, 24, 25]);
    }

    #[test]
    fn test_crossover() {
        let mut rng = StepRng::new(1, 2);
        let result = crossover(
            &[6, 2, 1, 4, 5, 3],
            &[5, 3, 1, 2, 4, 6],
            &mut rng
        );

        assert_eq!(result, vec![5, 2, 1, 4, 3, 6]);
    }

    #[test]
    fn test_crossover_at_start() {
        let mut rng = StepRng::new(0, 3);
        let result = crossover(
            &[6, 2, 1, 4, 5, 3],
            &[5, 3, 1, 2, 4, 6],
            &mut rng
        );

        assert_eq!(result, vec![6, 2, 1, 5, 3, 4]);
    }

    #[test]
    fn test_crossover_at_end() {
        let mut rng = StepRng::new(2, 2);
        let result = crossover(
            &[6, 2, 1, 4, 5, 3],
            &[5, 3, 1, 2, 4, 6],
            &mut rng
        );

        assert_eq!(result, vec![2, 6, 1, 4, 5, 3]);
    }

    #[test]
    fn test_mutate() {
        let mut rng = StepRng::new(1, 2);
        let mut genome: [u8; 6] = [1, 2, 3, 4, 5, 6];

        mutate(&mut genome, &mut rng);

        assert_eq!(genome, [1, 4, 3, 2, 5, 6]);
    }

    #[test]
    fn test_fitness() {
        let genome: [u8; 4] = [0, 2, 1, 3];
        let map = map::Map::new(50, 50, 4, map::LocationLayout::Random);

        let expected = 1.0 / (
            distance(&map.coords(0), &map.coords(2)) +
            distance(&map.coords(2), &map.coords(1)) +
            distance(&map.coords(1), &map.coords(3)) +
            distance(&map.coords(3), &map.coords(0))
        );
        
        assert_eq!(fitness(&genome, &map), expected);
    }

    #[test]
    fn test_distance_same_point() {
        let point = [5, 5];
        assert_eq!(distance(&point, &point), 0.0);
    }

    #[test]
    fn test_distance_horizontal() {
        let point_a = [0, 0];
        let point_b = [5, 0];
        assert_eq!(distance(&point_a, &point_b), 5.0);
    }

    #[test]
    fn test_distance_vertical() {
        let point_a = [0, 0];
        let point_b = [0, 5];
        assert_eq!(distance(&point_a, &point_b), 5.0);
    }

    #[test]
    fn test_distance_diagonal() {
        let point_a = [0, 0];
        let point_b = [3, 4];
        assert_eq!(distance(&point_a, &point_b), 5.0); // 3-4-5 triangle
    }

    #[test]
    fn test_distance_max_values() {
        let point_a = [0, 0];
        let point_b = [u8::MAX, u8::MAX];
        let expected_distance = ((u8::MAX as f32) * (u8::MAX as f32) * 2.0).sqrt();
        assert_eq!(distance(&point_a, &point_b), expected_distance);
    }

    #[test]
    fn test_distance_max_x() {
        let point_a = [u8::MAX, 0];
        let point_b = [0, 0];
        assert_eq!(distance(&point_a, &point_b), u8::MAX as f32);
    }

    #[test]
    fn test_distance_max_y() {
        let point_a = [0, u8::MAX];
        let point_b = [0, 0];
        assert_eq!(distance(&point_a, &point_b), u8::MAX as f32);
    }

    #[test]
    fn test_distance_mixed() {
        let point_a = [2, 3];
        let point_b = [8, 6];
        let expected_distance = ((((8 - 2) as i32).pow(2) + ((6 - 3) as i32).pow(2)) as f32).sqrt();
        assert_eq!(distance(&point_a, &point_b), expected_distance);
    }

    #[test]
    fn test_distance_wraparound() {
        let point_a = [255, 1];
        let point_b = [0, 1];
        assert_eq!(distance(&point_a, &point_b), 255.0);
    }
}