use crate::Examples::example_struct::Work;

pub fn run() -> Result<(), String> {
    let obj = Work::new(0)?;
    obj.method()?;
    Ok(())
}
