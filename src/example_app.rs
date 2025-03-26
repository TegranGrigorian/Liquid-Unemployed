use crate::example_struct::Work;

pub fn run() {
    let obj = Work::new(42);
    obj.method();
}
