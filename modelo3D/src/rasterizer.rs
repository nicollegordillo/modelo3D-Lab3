// rasterizer.rs
use crate::{fragment::Fragment, vertex::Vertex};
use nalgebra_glm::{Vec2, Vec3};


use nalgebra::{Vector2, Vector3};

// Assuming 2D points for pixel positions
fn compute_barycentric(p: &Vector2<f32>, v0: &Vector2<f32>, v1: &Vector2<f32>, v2: &Vector2<f32>) -> Option<Vector3<f32>> {
    // Convert to 3D vectors by adding a Z-component of 0
    let v0p = Vector3::new(p.x - v0.x, p.y - v0.y, 0.0);
    let v0v1 = Vector3::new(v1.x - v0.x, v1.y - v0.y, 0.0);
    let v0v2 = Vector3::new(v2.x - v0.x, v2.y - v0.y, 0.0);

    // Compute dot products for the barycentric coordinates
    let d00 = v0v1.dot(&v0v1);
    let d01 = v0v1.dot(&v0v2);
    let d11 = v0v2.dot(&v0v2);
    let d20 = v0p.dot(&v0v1);
    let d21 = v0p.dot(&v0v2);

    let denom = d00 * d11 - d01 * d01;

    if denom == 0.0 {
        return None; // Degenerate triangle
    }

    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = 1.0 - v - w;

    Some(Vector3::new(u, v, w))
}



// Function to determine if a point is inside a triangle using barycentric coordinates
fn point_in_triangle(pt: &Vec2, v0: &Vec2, v1: &Vec2, v2: &Vec2) -> bool {
    let v0v1 = v1 - v0;
    let v0v2 = v2 - v0;
    let v0p = pt - v0;

    let dot00 = v0v1.dot(&v0v1);
    let dot01 = v0v1.dot(&v0v2);
    let dot02 = v0v1.dot(&v0p);
    let dot11 = v0v2.dot(&v0v2);
    let dot12 = v0v2.dot(&v0p);

    // Barycentric coordinates
    let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
    let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
    let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;

    u >= 0.0 && v >= 0.0 && u + v <= 1.0
}

// Function to draw and fill a triangle
pub fn draw_filled_triangle(
    v0: &Vertex,
    v1: &Vertex,
    v2: &Vertex,
    z_buffer: &mut Vec<f32>,
    framebuffer_width: usize,
    framebuffer_height: usize, // Add height parameter
) -> Vec<Fragment> {
    let mut fragments = Vec::new();

    // Calculate the bounding box
    let min_x = v0.position.x.min(v1.position.x).min(v2.position.x).floor() as i32;
    let max_x = v0.position.x.max(v1.position.x).max(v2.position.x).ceil() as i32;
    let min_y = v0.position.y.min(v1.position.y).min(v2.position.y).floor() as i32;
    let max_y = v0.position.y.max(v1.position.y).max(v2.position.y).ceil() as i32;

    // Iterate through each pixel in the bounding box
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            // Check if the pixel is within the framebuffer bounds
            if x < 0 || x >= framebuffer_width as i32 || y < 0 || y >= framebuffer_height as i32 {
                continue; // Skip out-of-bounds pixels
            }

            let pixel_pos = Vec2::new(x as f32, y as f32);

            // Check if the pixel is inside the triangle using barycentric coordinates
            if let Some(bary_coords) = compute_barycentric(
                &pixel_pos, 
                &v0.position.xy(),  // Use only the x and y components of the 3D position
                &v1.position.xy(),
                &v2.position.xy()
            ) {
                // Interpolate depth (Z) using the barycentric coordinates
                let depth = bary_coords.x * v0.position.z
                    + bary_coords.y * v1.position.z
                    + bary_coords.z * v2.position.z;

                // Calculate the Z-buffer index
                let index = (y as usize * framebuffer_width + x as usize) as usize;

                // Z-buffer check and update
                if depth < z_buffer[index] {
                    z_buffer[index] = depth;

                    // Interpolate color using the barycentric coordinates
                    let color = v0.color * bary_coords.x
                        + v1.color * bary_coords.y
                        + v2.color * bary_coords.z;

                    // Add fragment to the list
                    fragments.push(Fragment::new(x as f32, y as f32, color, depth));
                }
            }
        }
    }

    fragments
}

