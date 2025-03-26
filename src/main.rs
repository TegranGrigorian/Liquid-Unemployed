//This code will have the main function and pre-set disabled features becasue rust compiler is poopyy#![allow(non_snake_case)]#[allow(non_snake_case)
#![allow(non_snake_case)]
#[warn(non_camel_case_types)]
#[warn(dead_code)]

//imports
use Liquid_Unemployed::Examples::example_app::run;

//main
fn main() {
    if let Err(e) = run() {
        println!("Application error: {}", e);
    }
}
