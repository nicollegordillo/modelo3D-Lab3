// obj.rs

use nalgebra_glm::{Vec2, Vec3};
use crate::vertex::Vertex;
use crate::color::Color;
use tobj;


pub struct Obj {
    pub vertices: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub texcoords: Vec<Vec2>,
    pub indices: Vec<u32>,
}

impl Obj {
    pub fn load(filename: &str) -> Result<Self, tobj::LoadError> {
        let (models, _) = tobj::load_obj(filename, &tobj::LoadOptions {
            single_index: true,
            triangulate: true,
            ..Default::default()
        })?;

        let mesh = &models[0].mesh;

        let vertices: Vec<Vec3> = mesh.positions.chunks(3)
            .map(|v| Vec3::new(v[0], v[1], v[2]))
            .collect();

        let normals: Vec<Vec3> = mesh.normals.chunks(3)
            .map(|n| Vec3::new(n[0], n[1], n[2]))
            .collect();

        let texcoords: Vec<Vec2> = mesh.texcoords.chunks(2)
            .map(|t| Vec2::new(t[0], t[1]))
            .collect();

        let indices = mesh.indices.clone();

        Ok(Obj {
            vertices,
            normals,
            texcoords,
            indices,
        })
    }

    pub fn to_vertices(&self) -> Vec<Vertex> {
        let mut vertex_array = Vec::new();

        for &index in &self.indices {
            let i = index as usize;

            let position = self.vertices[i];
            let normal = if !self.normals.is_empty() {
                self.normals[i]
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            };

            let tex_coords = if !self.texcoords.is_empty() {
                self.texcoords[i]
            } else {
                Vec2::new(0.0, 0.0)
            };

            let color = Color::black();

            let vertex = Vertex::new_with_color(position, color);
            vertex_array.push(vertex);
        }

        vertex_array
    }
}


