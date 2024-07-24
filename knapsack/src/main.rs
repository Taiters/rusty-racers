use textplots::{Chart, Plot, Shape};

use std::io;
use std::fmt;
use std::io::Write;
use std::str::FromStr;
use rand::distributions::Distribution;
use rand::distributions::WeightedIndex;
use rand::Rng;

mod items;
mod population;

struct Settings {
    items: Vec<items::Item>,
    max_weight: u32,
    population_size: usize,
    generations: usize,
    crossover_rate: f32,
    mutation_rate: f32,
}

fn read_input<T: FromStr>(prompt: &str) -> T 
where T: FromStr, <T as FromStr>::Err : fmt::Debug
{
    let mut input = String::new();
    print!("{} ", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).expect("Expected input on stdin");
    input.trim().parse().unwrap()
}

impl Settings {
    fn from_user() -> Self {
        let item_count: usize = read_input("Enter item count:");
        let items = items::generate(item_count);
        println!("Starting items:");
        for item in items.iter() {
            println!("- {}", item);
        }
        let total_weight: u32 = items.iter().map(|i| i.weight).sum();
        println!("Sum of weights: {}\n", total_weight);
        let max_weight: u32 = read_input("Enter max weight:");

        let population_size: usize = read_input("Enter population size:");
        let generations: usize = read_input("Enter generations:");
        let crossover_rate: f32 = read_input("Enter crossover rate:");
        let mutation_rate: f32 = read_input("Enter mutation rate:");

        return Self {
            items,
            max_weight,
            population_size,
            generations,
            crossover_rate,
            mutation_rate,
        }
    }
}

fn fitness(chromosone: &[u8], items: &[items::Item], max_weight: u32) -> u32 {
    let total_weight: u32 = chromosone
        .iter()
        .zip(items)
        .map(|v| u32::from(*v.0) * v.1.weight)
        .sum();

    if total_weight > max_weight {
        return 0;
    }

    return chromosone
        .iter()
        .enumerate()
        .map(|(i, v)| items[i].value * u32::from(*v))
        .sum();
}

#[derive(Debug)]
struct NotEnoughPopulationError;
impl fmt::Display for NotEnoughPopulationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Not enough population with fitness above 0")
    }
}

fn select<'a>(population: &'a Vec<Vec<u8>>, fitness: &[u32]) -> Result<(&'a Vec<u8>, &'a Vec<u8>), NotEnoughPopulationError> {
    let mut rng = rand::thread_rng();
    let non_zero_fitness = fitness.iter().filter(|&&f| f > 0).count();
    if non_zero_fitness <= 1 {
        return Err(NotEnoughPopulationError);
    }
    let dist = WeightedIndex::new(fitness).unwrap();

    let first_index = dist.sample(&mut rng);
    let first = population.get(first_index).unwrap();

    let second_index = loop {
        let index = dist.sample(&mut rng);
        if index != first_index {
            break index;
        }
    };
    let second = population.get(second_index).unwrap();

    return Ok((first, second));
}

fn crossover(a: &Vec<u8>, b: &Vec<u8>, crossover_rate: f32) -> (Vec<u8>, Vec<u8>) {
    if rand::random::<f32>() <= crossover_rate {
        let index = rand::random::<usize>() % a.len();
        let child1 = [&a[..index], &b[index..]].concat();
        let child2 = [&b[..index], &a[index..]].concat();
        return (child1, child2);
    }
    return (a.clone(), b.clone())
}

fn mutate(chromosone: &mut [u8], mutation_rate: f32) {
    let mut rng = rand::thread_rng();
    for i in 0..chromosone.len() {
        if rng.gen::<f32>() <= mutation_rate {
            chromosone[i] = (i32::from(chromosone[i]) - 1).abs() as u8;
        }
    }
}


fn main() {
    let settings = Settings::from_user();
    let mut pop = population::generate(settings.population_size, settings.items.len());
    let mut points: Vec<(f32, f32)> = Vec::with_capacity(settings.generations);

    for gen in 0..settings.generations {
        let fitnesses = pop.iter()
            .map(|c| fitness(c, &settings.items, settings.max_weight))
            .collect::<Vec<u32>>();

        let best_fitness = fitnesses.iter().max().unwrap();
        println!("Best fitness for generation {gen}: {best_fitness}");

        points.push((gen as f32, *best_fitness as f32));

        let mut next_generation = Vec::with_capacity(settings.population_size);
        while next_generation.len() < settings.population_size {
            let parents = select(&mut pop, &fitnesses).unwrap();
            let mut children = crossover(&parents.0, &parents.1, settings.crossover_rate);
            mutate(&mut children.0, settings.mutation_rate);
            mutate(&mut children.1, settings.mutation_rate);
            next_generation.push(children.0);
            next_generation.push(children.1);
        }
        next_generation.truncate(settings.population_size);
        pop = next_generation;
    }

    let fitnesses = pop.iter()
        .map(|c| fitness(c, &settings.items, settings.max_weight))
        .collect::<Vec<u32>>();

    let winner = pop.iter().zip(fitnesses).max_by_key(|v| v.1).unwrap();
    let weight: u32 = winner.0.iter()
        .zip(&settings.items)
        .map(|v| u32::from(*v.0) * v.1.weight)
        .sum();

    println!("\n===The results are in!===\n");
    println!("Solution: {:?}", winner.0);
    println!("Fitness: {}", winner.1);
    println!("Weight: {}/{}", weight, settings.max_weight);

    Chart::new(120, 60, 0.0, settings.generations as f32)
        .lineplot(&Shape::Points(&points))
        .display();
}