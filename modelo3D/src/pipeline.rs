use crate::{
    vertex::Vertex,
    vertex_shader::vertex_shader,
    rasterizer::draw_filled_triangle,
    fragment::Fragment,
    uniforms::Uniforms,
};

use crate::color::Color;
use nalgebra_glm::{Mat4, Vec3};

pub struct Pipeline {
    pub uniforms: Uniforms,
    pub light_position: Vec3, // Add a field for light position
}

impl Pipeline {
    pub fn new(transform_matrix: Mat4, light_position: Vec3) -> Self {
        Pipeline {
            uniforms: Uniforms {
                transform_matrix,
            },
            light_position,
        }
    }

    // Function to group vertices into triangles
    fn primitive_assembly(&self, vertices: &Vec<Vertex>) -> Vec<[Vertex; 3]> {
        let mut triangles = Vec::new();

        // Iterate through vertices in groups of 3
        for chunk in vertices.chunks(3) {
            if chunk.len() == 3 {
                let triangle: [Vertex; 3] = [chunk[0].clone(), chunk[1].clone(), chunk[2].clone()];
                triangles.push(triangle);
            }
        }

        triangles
    }

    // Function to calculate the flat shading color for a vertex
    fn flat_shading(&self, vertex: &Vertex) -> Color {
        let light_direction = (self.light_position - vertex.position).normalize();
        let intensity = vertex.normal.dot(&light_direction).max(0.0); // Ensure it's non-negative
        vertex.color * intensity // Modulate color by intensity
    }

    // Rasterization function to convert triangles into fragments with shading
    // In your rasterize function
    pub fn rasterize(&self, triangles: &[ [Vertex; 3]], z_buffer: &mut Vec<f32>, framebuffer_width: usize, framebuffer_height: usize) -> Vec<Fragment> {
        let mut fragments = Vec::new();

        for triangle in triangles {
            let triangle_fragments = draw_filled_triangle(
                &triangle[0], 
                &triangle[1], 
                &triangle[2], 
                z_buffer, 
                framebuffer_width, 
                framebuffer_height
            );
            fragments.extend(triangle_fragments);
        }

        fragments
    }


    // Run the pipeline process
    pub fn run(&self, vertices: &Vec<Vertex>, z_buffer: &mut Vec<f32>, framebuffer_width: usize, framebuffer_height: usize) -> Vec<Fragment> {
        // Group vertices into triangles
        let triangles = self.primitive_assembly(vertices);
    
        // Rasterize the triangles to obtain fragments
        let fragments = self.rasterize(&triangles, z_buffer, framebuffer_width, framebuffer_height);

        fragments
    }
}

