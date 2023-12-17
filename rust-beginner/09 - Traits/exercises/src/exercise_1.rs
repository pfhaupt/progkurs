trait Stats {
    fn get_weight(&self) -> f32;
    fn get_length(&self) -> f32;
}
struct Fish {
    kind: String,
    weight: f32,
    length: f32,
}
impl Stats for Fish {
    fn get_weight(&self) -> f32 {
        self.weight
    }
    fn get_length(&self) -> f32 {
        self.length
    }
}
pub fn main() {
    let fish = Fish {
        kind: String::from("Salmon"),
        weight: 10.0,
        length: 20.0,
    };
    println!("The {} is {}cm long and weighs {}kg",
             fish.kind, fish.get_length(), fish.get_weight());
}
