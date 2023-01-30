// lib.rs
mod cat;
mod dog;

pub use cat::Cat;
pub use dog::Dog;
pub use core::any::type_name;

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}