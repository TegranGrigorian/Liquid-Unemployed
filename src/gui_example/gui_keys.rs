use egui::Context;

#[derive(Default)]
pub struct ControlPanel {
    grid: bool,
    axes: bool
}

impl ControlPanel {
    pub fn show(&mut self, ctp: &Context) {
        egui::SidePanel::left("controls_panel")
        .resizable(false)
        .show(ctp, |ui| {
            ui.heading("3D Controls");
            ui.separator();
            
            ui.checkbox(&mut self.grid, "Show Grid");
            ui.checkbox(&mut self.axes, "Show Axes");
            
            ui.separator();
            if ui.button("Reset View").clicked() {
                //ur fucked
            }
        }); //one function btw
    }
}