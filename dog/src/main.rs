use std::ops::Add;

struct Dog {name: String,age: u32,}

//注意syntax: impl <trait> for <struct>
impl Add for Dog {
    // type 是什麼意思？
    type Output = u32;
    fn add(self, other: Dog) -> u32 {self.age + other.age}
}

fn main() {
    let dog1 = Dog {name: "Rufus".to_string(),age: 5};
    let dog2 = Dog {name: "Rusty".to_string(),age: 7,};
    // 這裡有 operator overload
    let total_age = dog1 + dog2;
    println!("The total age is {}", total_age);
}
