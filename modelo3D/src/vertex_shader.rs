// vertex_shader.rs

use nalgebra_glm::{Vec4, Vec3};
use crate::vertex::Vertex;
use crate::uniforms::Uniforms;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    // Expand the vertex position into a 4D vector
    let position_4d = Vec4::new(vertex.position.x, vertex.position.y, vertex.position.z, 1.0);

    // Apply the transformation matrix
    let transformed_position_4d = uniforms.transform_matrix * position_4d;

    // Reduce it back to a 3D vector
    let transformed_position = Vec3::new(
        transformed_position_4d.x,
        transformed_position_4d.y,
        transformed_position_4d.z,
    );

    let mut transformed_vertex = vertex.clone();
    transformed_vertex.set_transformed(transformed_position, vertex.normal);
    
    transformed_vertex
}
