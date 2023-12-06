use crate::shader::Shader;
use crate::shader::ShaderBase;
use crate::types::*;
use std::f32::consts::PI;

pub struct FlameTest {
    pub base: ShaderBase,
}

impl FlameTest {
    pub fn new((width, height): (u32, u32)) -> Self {
        let mut flame_test = FlameTest {
            base: ShaderBase::new(),
        };
        flame_test.base.set_resolution(width, height);
        flame_test
    }
	
}

	fn length2(p: Vec2) -> f32 {
    p.dot(&p)
}


fn rand(co: Vec2) -> f32 {
    (co.dot(&Vec2::new((12.9898, 78.233))) * 43758.5453).fract()
}

fn get_cell_point(cell: (i32, i32)) -> Vec2 {
    let cell_base = Vec2::new((cell.0 as f32, cell.1 as f32)) / 16.0;
    let noise_x = rand(Vec2::new((cell.0 as f32, cell.0 as f32)));
    let noise_y = rand(Vec2::new((cell.1 as f32, cell.1 as f32)));
    cell_base + (0.5 + 1.5 * Vec2::new((noise_x, noise_y))) / 16.0
}

fn worley(coord: Vec2) -> f32 {
    let cell = ((coord.x * 16.0) as i32, (coord.y * 16.0) as i32);
    let mut dist: f32 = 1.0;

    for x in 0..5 {
        for y in 0..5 {
            let cell_point = get_cell_point((cell.0 + x - 2, cell.1 + y - 2));
            dist = dist.min((cell_point - coord).length());
        }
    }

    dist /= Vec2::new((1.0 / 16.0, 1.0 / 16.0)).length();
    dist = 1.0 - dist;
    dist
}


fn fworley(p: Vec2, time: f32) -> f32 {
    let t1 = worley(p * 5.0 + Vec2::new((0.05 * time, 0.05 * time)));

    let t2 = worley(p * 50.0 + Vec2::new((0.12 + -0.1 * time, 0.12 + -0.1 * time)));

    let t3 = worley(p * -10.0 + Vec2::new((0.03 * time, 0.03 * time)));

    let sqrt_t2 = t2.sqrt();

    let sqrt_sqrt_t3 = t3.sqrt().sqrt();

    let result = (t1 * sqrt_t2 * sqrt_sqrt_t3).sqrt();

    result
}





impl Shader for FlameTest {
    fn base(&mut self) -> &mut ShaderBase {
        &mut self.base
    }

    
      fn main(&self, frag_coords: Vec2, _tex_coords: Option<Vec2>) -> Vec3 {
    let uv = frag_coords / self.base.resolution;

    let t = fworley(uv * self.base.resolution / 1500.0, self.base.game_time);

    let intensity = t * (length2((0.7 * uv - Vec2::new((1.0, 1.0))).abs()) * -1.0).exp();

    let final_color = Vec3::new((intensity * 0.1, intensity * 1.1 * t, intensity.powf(0.5 - t)));

    final_color
}

}
