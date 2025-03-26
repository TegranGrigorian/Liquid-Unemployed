use egui::{Ui, Pos2, Vec2, Response};
use std::collections::VecDeque;

//define our own Pos3 since egui is a bum
#[derive(Debug, Clone, Copy)]
pub struct Pos3 {
    pub x: f32,
    pub y: f32,
    pub z: f32, 
}

impl Pos3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { 
            x, 
            y, 
            z 
        }
    }
}

pub struct View3d {
    camera: CameraState,
    drawing: VecDeque<Pos3>,
    last_mouse_pos: Pos2, //lmao i never made camera movement code, idc stfu
}

impl View3d {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            camera: CameraState::default(),
            drawing: VecDeque::new(),
            last_mouse_pos: Pos2::ZERO,
        }
    }

    //NIGHTMARE I HATE RUSTS BORROW CHECKER WHY DIDNT I DO THIS IN C FUCKE MEEEEE
    fn handle_interaction(&mut self, response: &mut Response) { //we wnat to make the Response struct mutable
        if response.dragged() {
            let delta = response.drag_delta();
            self.camera.rotate(delta.x as f64, delta.y as f64);
            
            response.mark_changed();
        }
        
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                self.drawing.push_back(self.screen_to_world(pos));
            }
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        // Corrected area creation
        let mut response = egui::Area::new("3D Viewport".into()) //this is disgusting code, moving response to heap but it works with no errors
            .show(ui.ctx(), |ui| {
                self.render_viewport(ui);  // No need to mutate `ui` here
            })
            .response;
        
        self.handle_interaction(&mut response); //take ownership its not leaving the scope
    }
    
    fn render_viewport(&self, ui: &mut Ui) {
        ui.painter().rect_filled(
            ui.available_rect_before_wrap(),
            0.0,
            egui::Color32::from_gray(20),
        );
    }
    
    fn screen_to_world(&self, screen_pos: Pos2) -> Pos3 {
        Pos3::new(screen_pos.x, screen_pos.y, 0.0)
    }
}

//I really want to name these varibels and instances something dumb but ill be a nice boy
#[derive(Default)]
pub struct CameraState {
    x_rotation: f64,
    y_rotation: f64,
}

impl CameraState {
    fn rotate(&mut self, dx: f64, dy: f64) {
        self.x_rotation += dx * 0.01;
        self.y_rotation += dy * 0.01;
    }
}