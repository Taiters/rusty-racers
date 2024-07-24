use rand::Rng;

type Chromosone = Vec<u8>;

pub fn generate(size: usize, chromosone_length: usize) -> Vec<Chromosone> {
    let mut rng = rand::thread_rng();
    let mut population: Vec<Chromosone> = Vec::with_capacity(size);
    for _ in 0..size {
        let foo = Vec::from_iter((0..chromosone_length).map(|_| rng.gen_range::<u8, _>(0..2)));
        population.push(foo);
    }

    return population;
}
