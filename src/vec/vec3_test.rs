use vec::vec3::*;
use mat::mat3::*;
use std::f32::consts::*;

#[test]
fn creation() {	
	let mut my_vec: Vec3 = Vec3::new(2., 3., 4.);
	
	assert_eq!(my_vec.x, 2.);
	assert_eq!(my_vec.y, 3.);
	assert_eq!(my_vec.z, 4.);
	
	my_vec.set(4., 5., 6.);
	
	assert_eq!(my_vec.x, 4.);
	assert_eq!(my_vec.y, 5.);
	assert_eq!(my_vec.z, 6.);
	
	let my_vec3: Vec3 = Vec3::new_empty();
	
	assert_eq!(my_vec3.x, 0.);
	assert_eq!(my_vec3.y, 0.);
	assert_eq!(my_vec3.z, 0.);
}

#[test]
fn comparision() {	
	let mut my_vec: Vec3 = Vec3::new(2., 3., 4.);
	let base_vec: Vec3 = Vec3::new(4., 5., 6.);
	
	my_vec.set_from(&base_vec);
	
	let my_vec3: Vec3 = Vec3::new_empty();

	assert_eq!(my_vec == my_vec3, false);
	assert_eq!(my_vec != my_vec3, true);
	
	assert_eq!(my_vec == my_vec, true);
	assert_eq!(my_vec != my_vec, false);
	
	my_vec.set(0.,0.,0.);
	
	assert_eq!(my_vec == my_vec3, true);
	assert_eq!(my_vec != my_vec3, false);
}

#[test]
fn cross() {	
	let my_vec: Vec3 = Vec3::new(1.0, -5.0, 2.0);
	let my_vec3: Vec3 = Vec3::new(2.0, 0.0, 3.0);
	
	let result: Vec3 = my_vec.cross(&my_vec3);
	
	assert_eq!(result.x, -15.);
	assert_eq!(result.y, 1.);
	assert_eq!(result.z, 10.);
}

#[test]
fn mul_scalar() {	
	let mut my_vec: Vec3 = Vec3::new(2., 3., 4.) * 2.;
	
	assert_eq!(my_vec.x, 4.);
	assert_eq!(my_vec.y, 6.);
	assert_eq!(my_vec.z, 8.);
	
	my_vec *= 2.;
	
	assert_eq!(my_vec.x, 8.);
	assert_eq!(my_vec.y, 12.);
	assert_eq!(my_vec.z, 16.);
}

#[test]
fn mul_vec3() {	
	let result: f32 = Vec3::new(2., 3., 4.) * Vec3::new(3., 4., 5.);
	
	assert_eq!(result, 38.);
	
}

#[test]
fn is_orthogonal() {	
	let v1 = Vec3::new(0., 1., 0.);
	let v2 = Vec3::new(1., 0., 0.);
	
	assert!(v1.is_orthogonal(&v2));
	
	let v3 = Vec3::new(12., 23., 24.);
	
	assert!(!v1.is_orthogonal(&v3));
}

#[test]
fn len() {	
	let len = Vec3::new(5., 6., 7.).len();
	assert_eq!(len, 10.488089);
}

#[test]
fn clone() {	
	let v1 = Vec3::new(2., 2., 2.);
	let v2 = v1.clone();
	assert!(v1 == v2);
}

#[test]
fn copy_to() {	
	let v1 = Vec3::new(2., 2., 2.);
	let mut v2 = Vec3::new_empty();
	v1.copy_to(&mut v2);
	assert!(v1 == v2);
}

#[test]
fn negation() {	
	let mut my_vec: Vec3 = Vec3::new(2., 3., 4.);
	
	my_vec.negate();
	
	let mut my_vec3: Vec3 = Vec3::new(-2., -3., -4.);
	assert_eq!(my_vec == my_vec3, true);
	
	my_vec.set(23., 23., 23.);
	
	my_vec.negate_to(&mut my_vec3);
	assert_eq!(my_vec3.x, -23.);
	assert_eq!(my_vec3.y, -23.);
	assert_eq!(my_vec3.z, -23.);
}

#[test]
fn add_scalar() {	
	let mut my_vec: Vec3 = Vec3::new(2., 3., 4.) + 2.;
	
	assert_eq!(my_vec.x, 4.);
	assert_eq!(my_vec.y, 5.);
	assert_eq!(my_vec.z, 6.);
	
	my_vec += -2.;
	
	assert_eq!(my_vec.x, 2.);
	assert_eq!(my_vec.y, 3.);
	assert_eq!(my_vec.z, 4.);
}

#[test]
fn add_vec() {	
	let mut my_vec: Vec3 = Vec3::new(2., 3., 4.) + Vec3::new(2., 2., 2.);
	
	assert_eq!(my_vec.x, 4.);
	assert_eq!(my_vec.y, 5.);
	assert_eq!(my_vec.z, 6.);
	
	my_vec += Vec3::new(-2., -2., -2.);
	
	assert_eq!(my_vec.x, 2.);
	assert_eq!(my_vec.y, 3.);
	assert_eq!(my_vec.z, 4.);
}

#[test]
fn add_vec_ref() {	
	let mut my_vec: Vec3 = Vec3::new(2., 3., 4.);
	let my_vec2: Vec3 = Vec3::new(2., 2., 2.);
	let my_vec3: Vec3 = Vec3::new(-2., -2., -2.);
	 
	let result: Vec3 = &my_vec + &my_vec2;
	
	assert_eq!(result.x, 4.);
	assert_eq!(result.y, 5.);
	assert_eq!(result.z, 6.);
	
	my_vec += &my_vec3;
	
	assert_eq!(my_vec.x, 0.);
	assert_eq!(my_vec.y, 1.);
	assert_eq!(my_vec.z, 2.);
}

#[test]
fn sub_scalar() {	
	let mut my_vec: Vec3 = Vec3::new(2., 3., 4.) - 2.;
	
	assert_eq!(my_vec.x, 0.);
	assert_eq!(my_vec.y, 1.);
	assert_eq!(my_vec.z, 2.);
	
	my_vec -= 2.;
	
	assert_eq!(my_vec.x, -2.);
	assert_eq!(my_vec.y, -1.);
	assert_eq!(my_vec.z, 0.);
}

#[test]
fn sub_vec() {	
	let mut my_vec: Vec3 = Vec3::new(2., 3., 4.) - Vec3::new(2., 2., 2.);
	
	assert_eq!(my_vec.x, 0.);
	assert_eq!(my_vec.y, 1.);
	assert_eq!(my_vec.z, 2.);
	
	my_vec -= Vec3::new(2., 2., 2.);
	
	assert_eq!(my_vec.x, -2.);
	assert_eq!(my_vec.y, -1.);
	assert_eq!(my_vec.z, 0.);
}

#[test]
fn sub_vec_ref() {	
	let mut my_vec: Vec3 = Vec3::new(2., 3., 4.);
	let my_vec2: Vec3 = Vec3::new(2., 2., 2.);
	let my_vec3: Vec3 = Vec3::new(-2., -2., -2.);
	 
	let result: Vec3 = &my_vec - &my_vec2;
	
	assert_eq!(result.x, 0.);
	assert_eq!(result.y, 1.);
	assert_eq!(result.z, 2.);
	
	my_vec -= &my_vec3;
	
	assert_eq!(my_vec.x, 4.);
	assert_eq!(my_vec.y, 5.);
	assert_eq!(my_vec.z, 6.);
}

#[test]
fn to_string() {	
	let my_vec: Vec3 = Vec3::new(2., 3., 4.);
	let vec_string: String = my_vec.to_string();
	assert_eq!(vec_string, "x:2 y:3 z:4");
}

#[test]
fn normalize() {	
	let mut my_vec: Vec3 = Vec3::new(2., 3., 4.);
	my_vec.normalize();

	assert_eq!(my_vec.x, 0.37139067);
	assert_eq!(my_vec.y, 0.5570860);
	assert_eq!(my_vec.z, 0.74278134);
	
	assert_eq!(my_vec.len(), 1.);
}

#[test]
fn normalize_to() {	
	let my_vec: Vec3 = Vec3::new(2., 3., 4.);
	let mut my_vec3 = Vec3::new_empty();
	
	my_vec.normalize_to(&mut my_vec3);
	
	assert_eq!(my_vec3.x, 0.37139067);
	assert_eq!(my_vec3.y, 0.5570860);
	assert_eq!(my_vec3.z, 0.74278134);
	
	assert_eq!(my_vec3.len(), 1.);
}

#[test]
fn angle() {	
	let my_vec: Vec3 = Vec3::new(0., 1., 0.);
	let my_vec3: Vec3 = Vec3::new(1., 0., 0.);
	
	let angle_rad: f32 = my_vec.angle(&my_vec3);
	let angle_degree: f32 = angle_rad * (180. / PI);
	
	assert_eq!(angle_degree, 90.0);
}

#[test]
fn spat() {	
	let my_vec: Vec3 = Vec3::new(2., 0., 5.);
	let my_vec2: Vec3 = Vec3::new(-1., 5., -2.);
	let my_vec3: Vec3 = Vec3::new(2., 1., 2.);
	
	let spat: f32 = my_vec.spat(&my_vec2, &my_vec3);
	
	assert_eq!(spat, -31.0);
}

#[test]
fn mul_mat3() {	
	let mat: Mat3 = Mat3::new( 1., 0., 2., 0., 0., 4., 1., -1., 1.);
	let mat2: Mat3 = mat.clone();
	
	let vec: Vec3 = Vec3::new(2., 2., 0.);
	let vec_result: Vec3 = Vec3::new(2., 0., 0.);
	
	let vec_result_2 = &vec * &mat;
	assert_eq!(vec_result_2 == vec_result, true);
	
	let mut vec_2 = vec.clone();
	vec_2 *= &mat;
	assert_eq!(vec_2 == vec_result, true);
	
	let mut vec_3 = vec.clone();
	vec_3 *= mat;
	assert_eq!(vec_3 == vec_result, true);
	
	vec_3 = vec * mat2;
	assert_eq!(vec_3 == vec_result, true);
}

#[test]
fn mul_mat_rotx() {	
	let rotx: Mat3 = Mat3::new_rotx(90.);
	let roty: Mat3 = Mat3::new_roty(90.);
	let rotz: Mat3 = Mat3::new_rotz(90.);
	
	let mut vec: Vec3 = Vec3::new(1., 0., 0.);
	let mut vec2: Vec3 = vec.clone();
	let mut vec3: Vec3 = vec.clone();
	
	
	vec *= &rotx;
	assert_eq!(vec == vec2, true);
	
	vec2 *= &roty;
	assert_eq!(vec2.z, -1.);
	
	vec3 *= &rotz;
	assert_eq!(vec3.y, 1.);
	
}