Rust Shader Rendering Library Proposal
Name: Alex Fevery
Email Address: fevery@pdx.edu
Project Name: RustShader
Project Topic Area:
Shader Rendering and Image Processing using Rust
Project Vision:
"RustShader" will be a library I create in Rust. It allows users to input pixel shaders, similar to 
those from platforms like Shadertoy, and render a frame which is then saved as an image. 
Examples of simple shaders can be seen here:
Key Features:
CPU-based Rendering: Instead of using a GPU, RustShader will use the CPU to render shaders. 
This means it can run on systems without a GPU.
Pixel Computation: The library will calculate texture coordinates for each pixel in an empty 
image, using its x and y integers and a projection matrix. These coordinates will be fed through 
the shader written by the user.
Multithreading: I will use multithreading to handle multiple pixels at the same time and speed 
up image rendering. 
Writing Shaders in Rust: To start, the library will allow users to write shaders directly in Rust. It 
will provide functions similar to those in GLSL (like mix, dot, and clamp) for this purpose.
GLSL Parser (Stretch Goal): If the first part of the project is completed quickly, the next step is to 
make a GLSL parser that can read GLSL and convert it to my rust glsl functions automatically. 
This will allow a user to directly paste in glsl which would be really cool. 
Issues of Concern:
Performance: Since RustShader uses the CPU and not a GPU, it will be slow so that’s why it will 
only render a single frame and save it as a png. 
Converting GLSL to Rust: GLSL functions are very simple because they do very simple tasks (like 
mixing colors) so it shouldn’t be hard to replicate them in Rust. The automatic parsing of glsl 
might be hard because I have never written a parser before so I am setting that as a stretch goal.
