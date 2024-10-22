pub struct Space {
    pub name: String,
}

impl Space {
    pub fn new(name: &str) -> Space {
        Space {
            name: name.to_string(),
        }
    }
}
