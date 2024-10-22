pub struct InputError {
    text: String,
}

impl InputError {
    pub fn new(text: &str) -> InputError {
        InputError {
            text: text.to_string(),
        }
    }
}
