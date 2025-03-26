use egui::Context;

//hey is this code form someones github? yes stfu im lazy and need to see if my view math code works so fuck you
#[derive(Default)]
pub struct ControlPanel {
    grid: bool,
    axes: bool
}

impl ControlPanel {
    pub fn show(&mut self, ctp: &Context) {
        egui::SidePanel::left("Shitter Panel")
        .resizable(false)
        .show(ctp, |ui| {
            ui.heading("Controls im too lazy to program");
            ui.separator();
            
            ui.checkbox(&mut self.grid, "Show Grid");
            ui.checkbox(&mut self.axes, "Show Axes");
            
            ui.separator();
            if ui.button("Reset View").clicked() {
                println!("lmao get fucked u though this was gonna do shit loser");
                //ur fucked
            }
        }); //one function btw
    }
}