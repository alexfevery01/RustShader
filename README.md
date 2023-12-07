Rust Shader Rendering Library Proposal

Name: Alex Fevery

Email Address: fevery@pdx.edu

Project Name: RustShader

Project Topic Area:

Shader Rendering and Image Processing using Rust

Project Vision:

"RustShader" will be a library I create in Rust. It allows users to input pixel shaders, similar to 
those from platforms like Shadertoy, and render a frame which is then saved as an image. 


Key Features:

CPU-based Rendering:

Instead of using a GPU, RustShader will use the CPU to render shaders. 
This means it can run on systems without a GPU.

Pixel Computation:

The library will calculate texture coordinates for each pixel in an empty 
image, using its x and y integers and a projection matrix. These coordinates will be fed through 
the shader written by the user.

Multithreading(Stretch goal): 

I will use multithreading to handle multiple pixels at the same time and speed 
up image rendering. Not yet implemented.

Writing Shaders in Rust: 

To start, the library will allow users to write shaders directly in Rust. It 
will provide functions similar to those in GLSL (like mix, dot, and clamp) for this purpose.
GLSL Parser (Stretch Goal): If the first part of the project is completed quickly, the next step is to 
make a GLSL parser that can read GLSL and convert it to my rust glsl functions automatically. 
This will allow a user to directly paste in glsl which would be really cool. 

Issues of Concern:

Performance: 

Since RustShader uses the CPU and not a GPU, it will be slow so that’s why it will 
only render a single frame and save it as a png. 

Converting GLSL to Rust: 

GLSL functions are very simple because they do very simple tasks (like 
mixing colors) so it shouldn’t be hard to replicate them in Rust. The automatic parsing of glsl 
might be hard because I have never written a parser before so I am setting that as a stretch goal.

How It Works:

Input Shaders: Users can input pixel shaders written in Rust, utilizing functions similar to GLSL (e.g., mix, dot, clamp).

CPU Rendering: RustShader calculates texture coordinates for each pixel in an empty image, using integer values for pixels and a frag coord calculated from them. These coordinates are then processed through the user-provided shader. More advanced shaders could use projection matrixes. 

Building and Running the Project:
To build and run the project, follow these steps:

Clone the RustShader repository.

Navigate to the project directory.

Open a terminal and use "cargo build" to build the project

Run the project using "cargo run" to execute the shader rendering process.

Use "cargo test" to run the tests to check the glsl conversions.

Testing and issues Encountered:
Testing had to be done manually in most cases because of the very arbitrary values involved in the calculations of thousands of pixels that make up the images.  I did some testing on the Rust GLSL functions to make sure they produced the right values but the main issues seemed to be how they passed and combined values due to the very different syntax of rust and other langauges. 

Future improvements:
I did not reach my stretch goal of a rust glsl auto parser because I got stuck with my shaders not rendering correctly because of small differences in the way values are passed through the shader in rust due to its very different syntax and structure from other languages.  I may have implemented them by the time the assignment is graded though. 

Usage example:

    fn main() {
    let (width, height) = (800, 600); // Set resolution
    //use either the existing image library for the project or your own.  Create image. 
    let mut shader = SimpleTest::new((width, height)); // Choose a shader from the shaders folder and create it with the resolution
    for y in 0..height {
        for x in 0..width {
            let frag_coords = types::Vec2::new((x as f32 / width as f32, y as f32 / height as f32));
            let color = shader.main(frag_coords, None); // Execute shader
            // Process and set the pixel value at the position in the image (width/height)
        }
    }
    //save your image here.

    }




