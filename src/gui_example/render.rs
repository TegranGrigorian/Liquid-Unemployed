use super::view_math::CameraState;

pub struct Renderer3D;

impl Renderer3D {
    pub fn new(_gl: &glow::Context) -> Self {
        Self
    }
    
    pub fn resize(&mut self, _width: u32, _height: u32) {
        // Future implementation
    }
    
    pub fn render(&mut self, _camera: &CameraState) {
        // Future implementation
    }
}
//easy render code i stole