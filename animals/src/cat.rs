#[derive(Debug)]
pub struct Cat {
    pub name: String,
}

impl Cat {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
        }
    }
}