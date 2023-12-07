use crate::types::*;

pub struct ShaderBase {
    pub resolution: Vec2,
    pub game_time: f32,
}

impl ShaderBase {
    pub fn new() -> Self {
        ShaderBase {
            resolution: Vec2::new((0.0, 0.0)),
            game_time: 1.0,
        }
    }

    pub fn set_resolution(&mut self, width: u32, height: u32) {
        self.resolution = Vec2::new((width as f32, height as f32));
    }

    pub fn set_game_time(&mut self, game_time: f32) {
        self.game_time = game_time;
    }
}

pub trait Shader {
    fn base(&mut self) -> &mut ShaderBase;
    fn main(&self, frag_coords: Vec2, tex_coords: Option<Vec2>) -> Vec3;
    fn set_game_time(&mut self, game_time: f32);
}
