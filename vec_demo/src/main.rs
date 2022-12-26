// chat gpt prompt: write a rust program 
// exploring the functionality of vector

mod dog;
pub mod print_dir;

// use 

fn main() {
    println!("I am going to show you a Dog struct");
    println!("Then I'll call a static method (printer)");
    println!("Then I will showcase Vec");
    println!();

    //do dog things
    let dog = dog::new_dog("Marco".to_string());
    println!("The dog's name is {}", dog.get_name());
    dog.bark();
    println!();
    
    println!("Now calling the printer");
    //do static things
    print_dir::MyStruct::print_cur_dir();
    println!();
    
    // Create a new empty vector
    let mut vec = Vec::new();

    // Push some values onto the vector
    vec.push(1);
    vec.push(2);
    vec.push(3);

    // Print the vector
    println!("vec = {:?}", vec);

    // Get the length of the vector
    println!("vec has {} elements", vec.len());

    // Access elements of the vector using indexing
    println!("The first element is {}", vec[0]);

    // Get an iterator over the vector
    let iter = vec.iter();
    for val in iter {
        println!("Got value: {}", val);
    }
}
