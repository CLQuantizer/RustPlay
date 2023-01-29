use animals::{Cat, Dog};
fn main() {
    let cat = Cat::new("Mittens");
    let dog = Dog::new("Fido");
    println!("Cat: {}", cat.name);
    println!("Dog: {}", dog.name);
}
