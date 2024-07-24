use wasm_bindgen::prelude::*;
use rand::Rng;

pub mod map;
pub mod genomes;

#[wasm_bindgen]
pub struct Genome {
    pub fitness: f32,
    pub data: *const u8,
}

#[wasm_bindgen]
pub struct World {
    map: map::Map,
    population: Vec<u8>,
    fitnesses: Vec<f32>,
    crossover_rate: f32,
    mutation_rate: f32,
}

#[wasm_bindgen]
pub struct WorldSettings {
    pub width: u8,
    pub height: u8,
    pub locations: u8,
    pub population_size: usize,
    pub layout: map::LocationLayout,
    pub crossover_rate: f32,
    pub mutation_rate: f32,
}

#[wasm_bindgen]
impl WorldSettings {
    pub fn new(width: u8, height: u8, locations: u8, population_size: usize, layout: map::LocationLayout, crossover_rate: f32, mutation_rate: f32) -> Self {
        Self {
            width,
            height,
            locations,
            population_size,
            layout,
            crossover_rate,
            mutation_rate,
        }
    }
}

#[wasm_bindgen]
impl World {
    pub fn new(settings: &WorldSettings) -> Self {
        let mut rng = rand::thread_rng();
        let map = map::Map::new(settings.width, settings.height, settings.locations, settings.layout);
        let population = (0..settings.population_size)
            .flat_map(|_| genomes::generate(settings.locations, &mut rng))
            .collect::<Vec<u8>>();

        let fitnesses = population
            .chunks_exact(settings.locations as usize)
            .map(|g| genomes::fitness(g, &map))
            .collect::<Vec<f32>>();

        Self {
            map,
            population,
            fitnesses,
            crossover_rate: settings.crossover_rate,
            mutation_rate: settings.mutation_rate,
        }
    }

    pub fn fittest(&self) -> Genome {
        let length = self.location_count();
        let max_index = self.fitnesses
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.total_cmp(b))
            .map(|(i, _)| i)
            .unwrap();
        let fittest = &self.population[max_index*length..max_index*length + length];
        let highest_fitness = self.fitnesses[max_index];

        Genome {
            fitness: highest_fitness,
            data: fittest.as_ptr(),
        }
    }

    pub fn tick(&mut self) {
        let population_length = self.population.len();
        let location_count = self.location_count();
        let mut new_population: Vec<u8>= Vec::with_capacity(population_length);
        let mut rng = rand::thread_rng();

        while new_population.len() < population_length {
            let parents = genomes::select(&self.population, &self.fitnesses, &mut rng);
            if rng.gen::<f32>() <= self.crossover_rate {
                let (parent1, parent2) = parents.split_at(location_count);
                let mut child = genomes::crossover(parent1, parent2, &mut rng);
                if rng.gen::<f32>() <= self.mutation_rate {
                    genomes::mutate(&mut child, &mut rng);
                }
                new_population.append(&mut child);
            } else {
                for p in parents.chunks_exact(location_count) {
                    if rng.gen::<f32>() <= self.mutation_rate {
                        let mut copied = p.to_vec();
                        genomes::mutate(&mut copied, &mut rng);
                        new_population.append(&mut copied);
                    } else {
                        new_population.extend_from_slice(p);
                    }

                    if new_population.len() == population_length {
                        break;
                    }
                }
            }
        }

        self.population = new_population;
        self.fitnesses = self.population
            .chunks_exact(location_count)
            .map(|g| genomes::fitness(g, &self.map))
            .collect::<Vec<f32>>();
    }

    pub fn width(&self) -> u8 {
        self.map.width()
    }

    pub fn height(&self) -> u8 {
        self.map.height()
    }

    pub fn location_count(&self) -> usize {
        self.map.locations().len() / 2
    }

    pub fn locations(&self) -> *const u8 {
        self.map.locations().as_ptr()
    }

    pub fn population_size(&self) -> usize {
        self.population.len() / self.location_count()
    }

    pub fn population(&self) -> *const u8 {
        self.population.as_ptr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_works() {
        let mut world = World::new(&WorldSettings::new(
            255,
            255,
            255,
            2000,
            map::LocationLayout::Circle,
            0.3,
            0.01,
        ));

        world.tick();
    }

}