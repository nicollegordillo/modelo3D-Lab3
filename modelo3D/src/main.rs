mod vertex;
mod fragment;
mod rasterizer;
mod obj;
mod pipeline;
mod uniforms;
mod vertex_shader;
mod color;

use color::Color;
use nalgebra_glm::{Mat4, Vec3};
use vertex::Vertex;
use std::error::Error;
use minifb::{Key, Window, WindowOptions};

fn main() -> Result<(), Box<dyn Error>> {
    // Load your .obj file
    //let obj = obj::Obj::load("./spaceship.obj")?;
    
    // Convert loaded data to vertices
    //let vertices = obj.to_vertices();

    let vertices: Vec<Vertex> = vec![
        Vertex::new_with_color(Vec3::new(100.0, 100.0, 0.0), Color { r: 1.0, g: 0.0, b: 0.0 }), // Red
        Vertex::new_with_color(Vec3::new(400.0, 100.0, 0.0), Color { r: 0.0, g: 1.0, b: 0.0 }), // Green
        Vertex::new_with_color(Vec3::new(250.0, 500.0, 0.0), Color { r: 0.0, g: 0.0, b: 1.0 }), // Blue
    ];

    // Create a transform matrix (identity in this case)
    let transform_matrix = Mat4::identity();

    // Define a light position (example)
    let light_position = Vec3::new(1.0, 1.0, 1.0);

    // Create a pipeline
    let pipeline = pipeline::Pipeline::new(transform_matrix, light_position);
    
    // Set up window for rendering
    let window_width = 800;
    let window_height = 600;
    let mut buffer: Vec<u32> = vec![0; window_width * window_height];
    
    let mut window = Window::new(
        "3D Renderer",
        window_width,
        window_height,
        WindowOptions::default(),
    )?;

    // Main rendering loop
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Initialize a Z-buffer
        let mut z_buffer = vec![f32::INFINITY; window_width * window_height];

        // Run the pipeline to get fragments
        let fragments = pipeline.run(&vertices, &mut z_buffer, window_width, window_height);

        /*for fragment in &fragments {
            println!("Fragment Color: {:?}", fragment.color);
        }*/

        // Update buffer with fragment colors
        for fragment in fragments {
            let x = fragment.position.x as usize;
            let y = fragment.position.y as usize;

            if x < window_width && y < window_height {
                let index = y * window_width + x;

                // Assuming a simple color format, clamp values to [0, 255]
                buffer[index] = 
                    ((fragment.color.r.clamp(0.0, 1.0) * 255.0) as u32) << 16 | 
                    ((fragment.color.g.clamp(0.0, 1.0) * 255.0) as u32) << 8 | 
                    (fragment.color.b.clamp(0.0, 1.0) * 255.0) as u32;
            }
        }

        // Update the window with the rendered buffer
        window.update_with_buffer(&buffer, window_width, window_height)?;

        // Optionally: Clear buffer for next frame (if needed)
        buffer.fill(0);
    }

    Ok(())
}


