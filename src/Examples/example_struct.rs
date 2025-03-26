pub struct Work {
    pub test_val: i32,
}

impl Work {
    pub fn new(test_val: i32) -> Result<Self, &'static str> {
        if test_val == 0 {
            return Err("Error, value of 0 was inputted")
        } else {
            return Ok(Work { test_val })
        }
    }

    pub fn method(&self) -> Result<(), &'static str>{
        println!("Hi team!");
        if self.test_val == 0 { //just checking again
            return Err("Error, object failed");
        }

        Ok(())
    }
}
