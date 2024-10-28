use nalgebra_glm::{Vec3, dot};
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::line::line;
use crate::color::Color;

fn calculate_bounding_box(v1: &Vec3, v2: &Vec3, v3: &Vec3) -> (i32, i32, i32, i32) {
	let min_x = v1.x.min(v2.x).min(v3.x).floor() as i32;
	let min_y = v1.y.min(v2.y).min(v3.y).floor() as i32;
	let max_x = v1.x.max(v2.x).max(v3.x).ceil() as i32;
	let max_y = v1.y.max(v2.y).max(v3.y).ceil() as i32;

	(min_x, min_y, max_x, max_y)
}

pub fn triangle(v1: &Vertex, v2: &Vertex, v3: &Vertex) -> Vec<Fragment> {
	let mut fragments = Vec::new();

	let (a, b, c) = (v1.transformed_position, v2.transformed_position, v3.transformed_position);

	let (min_x, min_y, max_x, max_y) = calculate_bounding_box(&a, &b, &c);

	let area =edge_function(&a, &b, &c);
	let light_dir = Vec3::new(0.0, 0.0, -1.0);

	let color_a = Color::new(255.0,0.0,0.0);
	let color_b = Color::new(0.0,255.0,0.0);
	let color_c = Color::new(0.0,0.0,255.0);

	for y in min_y..max_y {
		for x in min_x..max_x {
			let p = Vec3::new(x as f32, y as f32, 0.0);

			let (u,v,w) = barycentric(&p,&a,&b,&c,area);

			if u < 0.0 || v <0.0 || w< 0.0 || u> 1.0 || v>1.0 || w >1.0{
				continue;
			}
			let normal = v1.normal * u + v2.normal * v + v3.normal * w;
			let normal = normal.normalize();
			let base_color = Color::new(100.0,100.0,100.0);
			//let base_color= color_a *u +color_b*v+color_c*w;
			let intensity = dot(&normal, &light_dir).max(0.0).min(1.0);

			if intensity < 0.0{
				continue;
			}

			let lit_color = base_color * intensity;

			let z = a.z * u + b.z * v + c.z * w;
			fragments.push(Fragment::new(p.x, p.y, lit_color, z));
				
		}
	}

	fragments
}

pub fn barycentric(p: &Vec3, a:&Vec3, b: &Vec3, c: &Vec3, area: f32) -> (f32, f32, f32) {
	/*let v0 = b-a;
	let v1 = c-a;
	let v2 = p-a;

	let d00 = dot(&v0, &v0);
	let d01 = dot(&v0, &v1);
	let d11 = dot(&v1, &v1);
	let d20 = dot(&v2, &v0);
	let d21 = dot(&v2, &v1);

	let denom = d00 * d11 - d01 * d01;

	let v = (d11 * d20 - d01 * d21) /denom;
	let w = (d00 * d21 - d01 * d20) /denom;
	let u = 1.0 - v - w;*/

	let u = edge_function(b,c,p) /area;
	let v = edge_function(c,a,p) /area;
	let w = edge_function(a,b,p) /area;

	(u,v,w)
}

fn edge_function(a: &Vec3, b: &Vec3, c: &Vec3) -> f32 {
	(c.x - a.x ) * (b.y - a.y) -(c.y-a.y ) *(b.x -a.x)
}