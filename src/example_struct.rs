pub struct Work {
    pub test_val: i32,
}

impl Work {
    pub fn new(test_val: i32) -> Self {
        Self { test_val }
    }

    pub fn method(&self) {
        println!("Hi team!");
    }
}
