use crate::shader::Shader;
use crate::shader::ShaderBase;
use crate::types::*;
use std::f32::consts::PI;

pub struct SimpleTest {
    pub base: ShaderBase,
    pub color1: Vec3,
    pub color2: Vec3,
    pub block_width: f32,
}

impl SimpleTest {
    pub fn new((width, height): (u32, u32)) -> Self {
        let mut shader = SimpleTest {
            base: ShaderBase::new(),
            color1: Vec3::new((0.0, 0.0, 0.3)),
            color2: Vec3::new((0.5, 0.0, 0.0)),
            block_width: 0.01,
        };
        shader.base.set_resolution(width, height);
        shader
    }

    fn step(&self, edge: f32, x: f32) -> f32 {
        if x < edge {
            0.0
        } else {
            1.0
        }
    }
}

impl Shader for SimpleTest {
    fn base(&mut self) -> &mut ShaderBase {
        &mut self.base
    }

    fn main(&self, frag_coords: Vec2, _tex_coords: Option<Vec2>) -> Vec3 {
        let uv = frag_coords / self.base.resolution;

        // Background pattern
        let color1 = Vec3::new((0.0, 0.0, 0.3));
        let color2 = Vec3::new((0.5, 0.0, 0.0));
        let block_width = 0.01;

        let mut c1 = (uv.x % (2.0 * block_width)) >= block_width;
        let mut c2 = (uv.y % (2.0 * block_width)) >= block_width;

        let bg_color = Vec3::mix(
            uv.x * color1,
            uv.y * color2,
            (c1 as i32 as f32) * (c2 as i32 as f32),
        );

        // Wave effect
        let mut wave_color = Vec3::new((0.0, 0.0, 0.0));
        let mut uv_mod = Vec2::new((-1.0, -1.0)) + Vec2::new((2.0, 2.0)) * uv;
        uv_mod.y += 0.1;
        for i in 0..10 {
            uv_mod.y += 0.07 * (uv_mod.x + i as f32 / 7.0 + self.base.game_time).sin();
            let wave_width = (150.0 * uv_mod.y).abs().recip();
            wave_color += Vec3::new((wave_width * 1.9, wave_width, wave_width * 1.5));
        }

        let final_color = bg_color + wave_color;

        Vec3::new((final_color.x, final_color.y, final_color.z))
    }
}
