use std::ops::Deref;

struct Thing<T>{
    value: T
}
impl <T> Deref for Thing<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
fn main() {
    println!("Hello, world!");
    let a = "Ezio".to_string();
    let thing: Thing<String> = Thing {value: a};
    assert_eq!("Ezio", thing.deref());
    assert_eq!("Ezio", *thing);
}
