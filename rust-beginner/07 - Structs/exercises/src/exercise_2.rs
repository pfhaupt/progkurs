struct Population {
    individuals: Vec<Individual>,
}
impl Population {
    fn get_average_age(&self) -> u32 {
        let mut sum = 0;
        for individual in &self.individuals {
            sum += individual.age;
        }
        sum / self.individuals.len() as u32
    }
}
struct Individual {
    age: u32,
    height: u32,
}
pub fn main() {
    let bob = Individual { age: 32, height: 172 };
    let alice = Individual { age: 27, height: 160 };
    let mut population = Population { individuals: vec![bob, alice] };
    population.individuals.push(Individual { age: 1, height: 50 });
    println!("Average age: {}", population.get_average_age());
}
