use std::f32::consts::PI;
use vec::vec3::*;
use mat::mat4::*;
extern crate rand;

#[derive(Clone)]
pub struct Barycentric {
	pub w0_12: f32,
	pub w1_20: f32,
	pub w2_01: f32,
	pub area: f32,
	pub bc0: f32,
	pub bc1: f32,
	pub bc2: f32,
	pub inside: bool,
}

impl Barycentric {
	pub fn new() -> Barycentric {
		Barycentric{ w0_12: 0., w1_20: 0., w2_01: 0., area: 0., bc0: 0., bc1: 0., bc2: 0., inside: false }
	}
}

const ONEDEGRAD: f32 = PI / 180.;

pub fn interpolate_lin( x: f32, x0: f32, f0: f32, x1: f32, f1: f32) -> f32{
	let mut result: f32 = 0.;
	
	if f0 != 0.0 {
		result += f0*((x1 - x)/(x1 - x0));
	} else {
		result = f0;
	}
	
	if f1 != 0.0 {
		result += f1*((x - x0)/(x1 - x0));
	}
	
	result
}

pub fn rad( degree: f32 ) -> f32 {
	ONEDEGRAD * degree
}

pub fn rand_path_deg( max_degree: f32) -> f32 {
	let deg: f32 = (rand::random::<f32>() % max_degree) + ((rand::random::<i32>() % 100) as f32 * 0.01);
	if rand::random::<i32>() % 2 == 0 { -deg } else { deg }
}

pub fn place_of_vec3( s: &Vec3, e: &Vec3, p: &Vec3) -> f32 {
	((p.x - s.x) * (e.y - s.y)) - ((p.y - s.y) * (e.x - s.x))
}

 
pub fn is_inside_triangle(_v0: &Vec3, _v1: &Vec3, _v2: &Vec3, _p: &Vec3, bc: &mut Barycentric ){
	bc.inside = false;
	
	bc.w0_12 = (_p.x - _v1.x) * (_v2.y - _v1.y) - (_p.y - _v1.y) * (_v2.x - _v1.x);
	if bc.w0_12 < 0. { return;}
	bc.w1_20 = (_p.x - _v2.x) * (_v0.y - _v2.y) - (_p.y - _v2.y) * (_v0.x - _v2.x);    
	if bc.w1_20 < 0. { return;}
	bc.w2_01 = (_p.x - _v0.x) * (_v1.y - _v0.y) - (_p.y - _v0.y) * (_v1.x - _v0.x);
	if bc.w2_01 < 0. { return;}

	//clockwise
	bc.inside = true; 

	bc.bc0 = bc.w0_12 / bc.area;
	bc.bc1 = bc.w1_20 / bc.area;
	bc.bc2 = bc.w2_01 / bc.area;	
}

pub fn transform_point( m: &Mat4, v: &mut Vec3) -> f32 {
	let mut temp: Vec3 = Vec3{
		x:(v.x * m._11) + (v.y * m._12) + (v.z * m._13) + m._14,
		y:(v.x * m._21) + (v.y * m._22) + (v.z * m._23) + m._24,
		z:(v.x * m._31) + (v.y * m._32) + (v.z * m._33) + m._34
	};
	let weight: f32 = (v.x * m._41) + (v.y * m._42) + (v.z * m._43) + m._44;
	if (weight != 1.) && (weight != 0.){
		temp.x /= weight;
		temp.y /= weight;
		temp.z /= weight;
	}
	v.x = temp.x;
	v.y = temp.y;
	v.z = temp.z;
	weight
}

pub fn transform_point_to( m: &Mat4, v: &Vec3, dest: &mut Vec3) -> f32 {
	dest.x = (v.x * m._11) + (v.y * m._12) + (v.z * m._13) + m._14;
	dest.y = (v.x * m._21) + (v.y * m._22) + (v.z * m._23) + m._24;
	dest.z = (v.x * m._31) + (v.y * m._32) + (v.z * m._33) + m._34;
	let mut weight: f32 = (v.x * m._41) + (v.y * m._42) + (v.z * m._43) + m._44;
	if (weight != 1.) && (weight != 0.){
		weight = 1./weight;
		dest.x *= weight;
		dest.y *= weight;
		dest.z *= weight;
	}
	weight
}

#[cfg(test)]
mod utils_test;