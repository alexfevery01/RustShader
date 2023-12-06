extern crate image;
use image::{ImageBuffer, Rgb};

mod shader;
mod types;
mod shaders {
    pub mod flame_test;
    pub mod simple_test;
}

use shader::*;
use shaders::flame_test::*;
use shaders::simple_test::*;
use types::*;

fn main() {
    let (width, height) = (600, 600); 
    let mut img = ImageBuffer::new(width, height);

    let mut shader1 = SimpleTest::new((width, height));

    for y in 0..height {
        for x in 0..width {
            let frag_coords = types::Vec2::new((x as f32, y as f32));
            let color = shader1.main(frag_coords, None);
            let scaled_color = Rgb([
                (color.x.clamp(0.0, 1.0) * 255.0) as u8,
                (color.y.clamp(0.0, 1.0) * 255.0) as u8,
                (color.z.clamp(0.0, 1.0) * 255.0) as u8,
            ]);
            img.put_pixel(x, y, scaled_color);
        }
    }

    img.save("shaderoutput.png")
        .unwrap();
}
