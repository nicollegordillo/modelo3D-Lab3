use nalgebra_glm::{Vec3, Mat4};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use triangle::triangle;
use shaders::vertex_shader;

pub struct Uniforms {
    model_matrix: Mat4,
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0, 0.0, 0.0, 0.0,
        0.0, cos_x, -sin_x, 0.0,
        0.0, sin_x, cos_x, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y, 0.0, sin_y, 0.0,
        0.0, 1.0, 0.0, 0.0,
        -sin_y, 0.0, cos_y, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z, cos_z, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0, 0.0, translation.x,
        0.0, scale, 0.0, translation.y,
        0.0, 0.0, scale, translation.z,
        0.0, 0.0, 0.0, 1.0,
    );
    
    transform_matrix * rotation_matrix
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex]){
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i+2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i+1].clone(),
                transformed_vertices[i+2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2]));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            let color = fragment.color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x , y, fragment.depth);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 600;
    let framebuffer_width = 800;
    let framebuffer_height = 600;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Rust 3D model",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500,500);
    window.update();

    framebuffer.set_background_color(0x333355);

    let mut translation = Vec3::new(300.0, 200.0, 0.0);
    let mut rotation = Vec3::new(0.0,0.0,0.0);
    let mut scale = 100.0f32;

    let obj = Obj::load("./spaceship.obj").expect("Failed to load obj");
    let vertex_arrays = obj.get_vertex_array();

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        handle_input(&window, &mut translation, &mut rotation, &mut scale);

        framebuffer.clear();

        let model_matrix = create_model_matrix(translation, scale, rotation);
        let uniforms= Uniforms { model_matrix };

        framebuffer.set_current_color(0xFFDDDD);
        render(&mut framebuffer, &uniforms, &vertex_arrays);

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn handle_input(window: &Window, translation: &mut Vec3, rotation: &mut Vec3, scale: &mut f32) {
  if window.is_key_down(Key::Right) {
    translation.x += 10.0;
  }
  if window.is_key_down(Key::Left) {
    translation.x -= 10.0;
  }
  if window.is_key_down(Key::Up) {
    translation.y -= 10.0;
  }
  if window.is_key_down(Key::Down) {
    translation.y += 10.0;
  }
  if window.is_key_down(Key::I) {
    *scale += 2.0;
  }
  if window.is_key_down(Key::O) {
    *scale -= 2.0;
  }
  if window.is_key_down(Key::D) {
        rotation.x -= PI / 10.0;
    }
    if window.is_key_down(Key::A) {
        rotation.x += PI / 10.0;
    }
    if window.is_key_down(Key::S) {
        rotation.y -= PI / 10.0;
    }
    if window.is_key_down(Key::W) {
        rotation.y += PI / 10.0;
    }
    if window.is_key_down(Key::Q) {
        rotation.z -= PI / 10.0;
    }
    if window.is_key_down(Key::E) {
        rotation.z += PI / 10.0;
    }
}
