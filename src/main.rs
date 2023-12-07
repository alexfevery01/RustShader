extern crate crossterm;
extern crate image;

use crossterm::event::{read, Event, KeyCode, KeyEvent};

use image::{
    gif::{GifEncoder, Repeat},
    Frame, ImageBuffer, Rgba,
};

use std::thread;
use std::time::Duration;

mod shader;
mod types;
mod shaders {
    pub mod test_shader01;
    pub mod test_shader02;
    pub mod test_shader03;
}

use shader::*;
use shaders::test_shader01::*;
use shaders::test_shader02::*;
use shaders::test_shader03::*;

fn main() {
    let output_file = std::fs::File::create("shaderoutput.gif").unwrap();

    let shaders = vec!["TestShader01", "TestShader02", "TestShader03"];
    let resolutions = vec!["4K", "1440p", "1080p", "720p", "480p", "360p", "240p"];
    let timestep_options = vec![".001 second", ".01 second", ".1 second"];
    let frames_options = vec![
        "4 frames",
        "8 frames",
        "16 frames",
        "30 frames",
        "60 frames",
        "200 frames",
    ];

    let mut selected_shader = 0;
    let mut selected_resolution = 0;
    let mut selected_timestep = 0;
    let mut selected_frames = 0;

    select_option("Select Shader:", &shaders, &mut selected_shader);
    select_option("Select Resolution:", &resolutions, &mut selected_resolution);
    select_option(
        "Select Timestep:",
        &timestep_options,
        &mut selected_timestep,
    );
    select_option(
        "Select Number of Frames:",
        &frames_options,
        &mut selected_frames,
    );

    let (width, height) = match selected_resolution {
        0 => (3840, 2160), // 4K
        1 => (2560, 1440), // 1440p
        2 => (1920, 1080), // 1080p
        3 => (1280, 720),  // 720p
        4 => (640, 480),   // 480p
        5 => (480, 360),   // 360p
        6 => (426, 240),   // 240p
        _ => panic!("Invalid resolution selected"),
    };

    let mut shader: Box<dyn Shader> = match selected_shader {
        0 => Box::new(TestShader01::new((width, height))),
        1 => Box::new(TestShader02::new((width, height))),
        2 => Box::new(TestShader03::new((width, height))),
        _ => panic!("Invalid shader selected"),
    };

    // Parse the timestep and frame count selections
    let timestep = match selected_timestep {
        0 => 0.001,
        1 => 0.01,
        2 => 0.1,
        _ => panic!("Invalid timestep selected"),
    };

    let frame_count = match selected_frames {
        0 => 4,
        1 => 8,
        2 => 16,
        3 => 30,
        4 => 60,
        5 => 200,
        _ => panic!("Invalid frame count selected"),
    };

    let mut encoder = GifEncoder::new_with_speed(output_file, 30);
    encoder.set_repeat(Repeat::Infinite).unwrap();

    for i in 0..frame_count {
        let mut img = ImageBuffer::new(width, height);
        shader.set_game_time(1.0 + timestep * i as f32);

        println!("Rendering frame {}/{}", i + 1, frame_count);

        for y in 0..height {
            for x in 0..width {
                let frag_coords = types::Vec2::new((x as f32, y as f32));
                let color = shader.main(frag_coords, None);
                let scaled_color = Rgba([
                    (color.x.clamp(0.0, 1.0) * 255.0) as u8,
                    (color.y.clamp(0.0, 1.0) * 255.0) as u8,
                    (color.z.clamp(0.0, 1.0) * 255.0) as u8,
                    255,
                ]);
                img.put_pixel(x, y, scaled_color);
            }
        }

        println!("Encoding frame {}/{}", i + 1, frame_count);
        let frame = Frame::new(img);
        encoder.encode_frame(frame).unwrap();
    }
    let binding = std::fs::canonicalize("shaderoutput.gif").unwrap();
    let path = binding.to_str().unwrap();
    println!(
        "Done! Output saved to {:?}",
        path.trim_start_matches(r"\\?\")
    );

    fn select_option(prompt: &str, options: &[&str], selected: &mut usize) {
        read().unwrap();
        loop {
            print!("\x1B[2J\x1B[1;1H");
            println!("Rust Shader");
            println!("***********");

            println!("");
            println!("");
            println!("{}", prompt);

            // Print options with the selected one marked
            for (i, option) in options.iter().enumerate() {
                if i == *selected {
                    println!("> {}", option);
                } else {
                    println!("  {}", option);
                }
            }
            println!("");
            println!("Up/Down arrow + ENTER to select");
            // Read keyboard input
            match read().unwrap() {
                Event::Key(KeyEvent { code, .. }) => {
                    match code {
                        KeyCode::Up => {
                            if *selected > 0 {
                                *selected -= 1;
                            }
                        }
                        KeyCode::Down => {
                            if *selected < options.len() - 1 {
                                *selected += 1;
                            }
                        }
                        KeyCode::Enter => break,
                        _ => {}
                    }
                    thread::sleep(Duration::from_millis(100));
                    read().unwrap();
                }
                _ => {}
            }
        }
    }
}
