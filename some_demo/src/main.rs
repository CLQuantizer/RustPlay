fn main() {
    let x = Some(10);
    let y : Option<u32>= None;

    match x {
        Some(val) => println!("x is some value: {}", val),
        None => println!("x is none"),
    }

    match y {
        Some(val) => println!("y is some value: {}", val),
        None => println!("y is none"),
    }
}
