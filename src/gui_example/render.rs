use super::view_math::CameraState;



//HAHAHAHAHA U THOUGHT I WAS GOING TO MAKE A RENDER ENGINE BY SCRATCH
/*
yeah helll no im going to steal one online, once i find one. I cant find one so for now everything looks liek shit womp womp bitch cry about it
also stop looking at the code u weirdo
*/
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