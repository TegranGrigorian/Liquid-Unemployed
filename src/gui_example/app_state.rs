//error checking for all the other classes
use egui::Context;
use crate::gui_example::{View3d, ControlPanel};

pub struct CadApp {
    viewport: View3d,
    controls: ControlPanel,
}

impl CadApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            viewport: View3d::new(cc),
            controls: ControlPanel::default(),
        }
    }
}

impl eframe::App for CadApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.viewport.show(ui);
        });
        
        self.controls.show(ctx);
    }
}