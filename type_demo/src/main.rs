use std::fmt;
use std::any::type_name;

struct Foo {
    a: i32,
    b: String,
}

impl fmt::Display for Foo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.a, self.b)
    }
}

fn main() {
    let x = Foo {
        a: 10,
        b: "Hello, world!".to_string(),
    };
    println!("{}", x);
    println!("The type of x is {}", type_name::<Foo>());
}
