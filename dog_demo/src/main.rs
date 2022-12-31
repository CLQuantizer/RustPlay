use std::ops::Add;
use std::ops::Sub;
use std::num::Wrapping;

#[derive(Copy, Clone)]
struct Dog {
    name: &'static str,
    age: u32,
}

//注意syntax: impl <trait> for <struct>
impl Add for Dog {
    // type 是什麼意思？
    type Output = u32;
    fn add(self, other: Dog) -> u32 {self.age + other.age}
}

impl Sub for Dog {
    type Output = u32;
    fn sub(self, other: Dog) -> u32 {
        if self.age > other.age { self.age - other.age } else { other.age - self.age }
    }
}

fn main() {
    let dog1 = Dog { name: "Rufus", age: 6 };
    let dog2 = Dog { name: "Rusty", age: 7 };
    // 這裡有 operator overload
    let total_age = dog1 + dog2;
    let age_diff = dog1 - dog2;
    println!("The total age is {}", total_age);
    println!("The age diff is {}", age_diff);
}