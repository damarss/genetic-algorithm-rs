use rand::Rng;

const POPULATION_SIZE: usize = 100;
const GENES: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ 1234567890,.-;:_!\"#%&/()=?@${[]}";
const TARGET: &str = "I love Stardew Valley";

fn main() {
    let mut generation = 0;
    let mut population: Vec<Individual> = Vec::new();

    // generate populasi awal 
    for _i in 0..POPULATION_SIZE {
        let gnome = create_gnome();
        population.push(Individual::from(gnome));
    }

    loop {
        // sort by smallest fitness
        population.sort_by(|a, b| a.fitness.cmp(&b.fitness));

        if population[0].fitness == 0 {
            break;
        }

        let mut new_generation: Vec<Individual> = Vec::new();

        // get elitism (best 10%)
        let s = (0.1 * POPULATION_SIZE as f32).floor() as usize;
        new_generation.extend_from_slice(&population[0..s]);

        let s = POPULATION_SIZE - s;
        let mut rng = rand::thread_rng();
        for _i in 0..s {
            let parent1 = &population[rng.gen_range(0..=50)];
            let parent2 = &population[rng.gen_range(0..=50)];
            let offspring = parent1.mate(parent2);
            new_generation.push(offspring);
        }

        population = new_generation;

        println!("Generation: {}\nString: {}\nFitness: {}", generation, population[0].chromosome, population[0].fitness);
        generation += 1;
    }

    println!("Generation: {}\nString: {}\nFitness: {}", generation, population[0].chromosome, population[0].fitness);
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
struct Individual {
    chromosome: String,
    fitness: u32
}

impl Individual {
    pub fn from(chromosome: String) -> Individual {
        let fitness = Self::calculate_fitness(&chromosome);

        Individual {
            chromosome: chromosome,
            fitness: fitness
        }
    }

    pub fn calculate_fitness(chromosome: &String) -> u32 {
        let mut fitness = 0;
        for i in 0..chromosome.len() {
            if chromosome.chars().nth(i).unwrap() != TARGET.chars().nth(i).unwrap() {
                fitness += 1;
            }
        }
        fitness
    }

    pub fn mate(&self, parent2: &Individual) -> Individual {
        let mut child_chromosome = String::new();
        for i in 0..self.chromosome.len() {
            let p: f32 = rand::thread_rng().gen();

            if p < 0.45 {
                child_chromosome.push(self.chromosome.chars().nth(i).unwrap());
            } else if p < 0.90 {
                child_chromosome.push(parent2.chromosome.chars().nth(i).unwrap());
            } else {
                child_chromosome.push(mutated_genes());
            }
        }
        Individual::from(child_chromosome)
    }
}


fn mutated_genes() -> char {
    GENES.chars().nth(rand::thread_rng().gen_range(0..GENES.len())).unwrap()
}

fn create_gnome() -> String {
    let len = TARGET.len();
    let mut gnome = String::new();
    for _i in 0..len {
        gnome.push(mutated_genes());
    }
    gnome
}
