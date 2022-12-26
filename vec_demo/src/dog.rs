pub struct Dog {
    name: String,
}

pub fn new_dog(name: String) -> Dog {
    Dog { name }
}

impl Dog {
    pub fn bark(&self) {
        println!("{} says bark!", self.name);
    }
    pub fn get_name(&self) -> &str{
        &self.name
    }
}

// fn main() {
//     let dog = Dog { name: "Buddy".to_string() };
//     dog.bark();
// }
