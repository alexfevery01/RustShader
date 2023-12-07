use crate::shader::Shader;
use crate::shader::ShaderBase;
use crate::types::*;

pub struct TestShader03 {
    pub base: ShaderBase,
    pub color1: Vec3,
    pub color2: Vec3,
    pub block_width: f32,
}

impl TestShader03 {
    pub fn new((width, height): (u32, u32)) -> Self {
        let mut shader = TestShader03 {
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

//ORIGINAL SHADER CREDIT: sincos
//https://www.shadertoy.com/view/WdjfzD

impl Shader for TestShader03 {
    fn base(&mut self) -> &mut ShaderBase {
        &mut self.base
    }

    fn set_game_time(&mut self, game_time: f32) {
        self.base.set_game_time(game_time);
    }

    fn main(&self, frag_coords: Vec2, _tex_coords: Option<Vec2>) -> Vec3 {
        let uv = frag_coords / self.base.resolution - Vec2::new((0.5, 0.5));

        let mut col = Vec4::zero();
        let thickness = 5.0 / self.base.resolution.y;

        for k in (0..=20).map(|k| k as f32 * 0.05) {
            if (uv.y - (k - 0.5) * (uv.x * 10.0 + 10.0 * (k - 0.5) * self.base.game_time).sin())
                .abs()
                < thickness * (k + 0.1)
            {
                col = Vec4::new((
                    1.0,
                    (3.14 * (1.0 - k) / 2.0).cos(),
                    (k * 3.14 / 2.0).cos(),
                    1.0,
                ));
            }
        }
        let col3 = Vec3::new((col.x, col.y, col.z));
        col3
    }
}
