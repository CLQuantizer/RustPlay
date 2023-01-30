use animals::{Cat, Dog, print_type_of};
fn main() {
    let cat = Cat::new("Mittens");
    let dog = Dog::new("Fido");
    print_type_of(&cat);
    println!("Dog: {}", dog.name);
    print_type_of(&print_type_of::<i32>); // playground::print_type_of<i32>
    print_type_of(&{ || "Hi!" }); // playground::main::{{closure}}
}