#[derive(Debug)]
pub struct Dog {
    pub name: String,
}

impl Dog {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}