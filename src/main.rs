//This code will have the main function and pre-set disabled features becasue rust compiler is poopyy#![allow(non_snake_case)]#[allow(non_snake_case)
#![allow(non_snake_case)]
#[warn(non_camel_case_types)]
#[warn(dead_code)]

//imports
// use Liquid_Unemployed::Examples::example_app::run;
use Liquid_Unemployed::gui_example::CadApp;
use eframe;
//main
fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Liquid-Unemployed"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Liquid-Unemployed",
        native_options,
        Box::new(|cc| Ok(Box::new(CadApp::new(cc)))),
    )
}