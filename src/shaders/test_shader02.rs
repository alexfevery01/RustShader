use crate::shader::Shader;
use crate::shader::ShaderBase;
use crate::types::*;

pub struct TestShader02 {
    pub base: ShaderBase,
    pub color1: Vec3,
    pub color2: Vec3,
    pub block_width: f32,
}

impl TestShader02 {
    pub fn new((width, height): (u32, u32)) -> Self {
        let mut shader = TestShader02 {
            base: ShaderBase::new(),
            color1: Vec3::new((0.0, 0.0, 0.3)),
            color2: Vec3::new((0.5, 0.0, 0.0)),
            block_width: 0.01,
        };
        shader.base.set_resolution(width, height);
        shader.base.set_game_time(1.0);
        shader
    }
}

//ORIGINAL SHADER CREDIT: DAVE HOSKINS
//https://www.shadertoy.com/view/4tjSDt

impl Shader for TestShader02 {
    fn base(&mut self) -> &mut ShaderBase {
        &mut self.base
    }

    fn set_game_time(&mut self, game_time: f32) {
        self.base.set_game_time(game_time);
    }

    fn main(&self, frag_coords: Vec2, _tex_coords: Option<Vec2>) -> Vec3 {
        let mut s = 0.0;
        let mut v = 0.0;
        let uv = (frag_coords / self.base.resolution) * 2.0 - Vec2::new(1.0);
        let time = (self.base.game_time - 2.0) * 58.0;
        let mut col = Vec3::new((0.0, 0.0, 0.0));
        let init = Vec3::new((
            (time * 0.0032).sin() * 0.3,
            0.35 - (time * 0.005).cos() * 0.3,
            time * 0.002,
        ));

        for _ in 0..100 {
            let mut p = init + s * Vec3::new((uv.x, uv.y, 0.05));
            p.z = p.z.fract();

            // Kali's chaotic loop
            for _ in 0..10 {
                p = (p.abs() * 2.04) / p.dot(&p) - 0.9;
            }

            v += p.dot(&p).powf(0.7) * 0.06;
            col += Vec3::new((v * 0.2 + 0.4, 12.0 - s * 2.0, 0.1 + v * 1.0)) * v * 0.00003;
            s += 0.025;
        }

        Vec3::new((
            col.x.clamp(0.0, 1.0),
            col.y.clamp(0.0, 1.0),
            col.z.clamp(0.0, 1.0),
        ))
    }
}
