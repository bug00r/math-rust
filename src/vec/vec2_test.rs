use vec::vec2::*;
use std::f32::consts::PI;

#[test]
fn creation() {	
	let mut my_vec: Vec2 = Vec2::new(2., 3.);
	
	assert_eq!(my_vec.x, 2.);
	assert_eq!(my_vec.y, 3.);
	
	my_vec.set(4.,5.);
	
	assert_eq!(my_vec.x, 4.);
	assert_eq!(my_vec.y, 5.);
	
	let my_vec2: Vec2 = Vec2::new_empty();
	
	assert_eq!(my_vec2.x, 0.);
	assert_eq!(my_vec2.y, 0.);

}

#[test]
fn comparision() {	
	let mut my_vec: Vec2 = Vec2::new(2., 3.);
	let base_vec: Vec2 = Vec2::new(4., 5.);
	
	my_vec.set_from(&base_vec);
	
	let my_vec2: Vec2 = Vec2::new_empty();

	assert_eq!(my_vec == my_vec2, false);
	assert_eq!(my_vec != my_vec2, true);
	
	assert_eq!(my_vec == my_vec, true);
	assert_eq!(my_vec != my_vec, false);
	
	my_vec.set(0.,0.);
	
	assert_eq!(my_vec == my_vec2, true);
	assert_eq!(my_vec != my_vec2, false);
}

#[test]
fn cross() {	
	let my_vec: Vec2 = Vec2::new(2., 3.);
	let my_vec2: Vec2 = Vec2::new(1., 2.);
	
	let result: f32 = my_vec.cross(&my_vec2);
	
	assert_eq!(result, 1.);
}

#[test]
fn mul_scalar() {	
	let mut my_vec: Vec2 = Vec2::new(2., 3.) * 2.;
	
	assert_eq!(my_vec.x, 4.);
	assert_eq!(my_vec.y, 6.);
	
	my_vec *= 2.;
	
	assert_eq!(my_vec.x, 8.);
	assert_eq!(my_vec.y, 12.);
}

#[test]
fn mul_vec2() {	
	let result: f32 = Vec2::new(2., 3.) * Vec2::new(3., 4.);
	
	assert_eq!(result, 18.);
	
}

#[test]
fn is_orthogonal() {	
	let v1 = Vec2::new(0., 1.);
	let v2 = Vec2::new(1., 0.);
	
	assert!(v1.is_orthogonal(&v2));
	
	let v3 = Vec2::new(12., 23.);
	
	assert!(!v1.is_orthogonal(&v3));
}

#[test]
fn len() {	
	let len = Vec2::new(2., 2.).len();
	assert_eq!(len, 2.828427);
}

#[test]
fn clone() {	
	let v1 = Vec2::new(2., 2.);
	let v2 = v1.clone();
	assert!(v1 == v2);
}

#[test]
fn copy_to() {	
	let v1 = Vec2::new(2., 2.);
	let mut v2 = Vec2::new_empty();
	v1.copy_to(&mut v2);
	assert!(v1 == v2);
}

#[test]
fn negation() {	
	let mut my_vec: Vec2 = Vec2::new(2., 3.);
	
	my_vec.negate();
	
	let mut my_vec2: Vec2 = Vec2::new(-2., -3.);
	assert_eq!(my_vec == my_vec2, true);
	
	my_vec.set(23., 23.);
	
	my_vec.negate_to(&mut my_vec2);
	assert_eq!(my_vec2.x, -23.);
	assert_eq!(my_vec2.y, -23.);
}

#[test]
fn add_scalar() {	
	let mut my_vec: Vec2 = Vec2::new(2., 3.) + 2.;
	
	assert_eq!(my_vec.x, 4.);
	assert_eq!(my_vec.y, 5.);
	
	my_vec += -2.;
	
	assert_eq!(my_vec.x, 2.);
	assert_eq!(my_vec.y, 3.);
}

#[test]
fn add_vec() {	
	let mut my_vec: Vec2 = Vec2::new(2., 3.) + Vec2::new(2., 2.);
	
	assert_eq!(my_vec.x, 4.);
	assert_eq!(my_vec.y, 5.);
	
	my_vec += Vec2::new(-2., -2.);
	
	assert_eq!(my_vec.x, 2.);
	assert_eq!(my_vec.y, 3.);
}

#[test]
fn add_vec_ref() {	
	let mut my_vec: Vec2 = Vec2::new(2., 3.);
	let my_vec2: Vec2 = Vec2::new(2., 2.);
	let my_vec3: Vec2 = Vec2::new(-2., -2.);
	 
	let result: Vec2 = &my_vec + &my_vec2;
	
	assert_eq!(result.x, 4.);
	assert_eq!(result.y, 5.);
	
	my_vec += &my_vec3;
	
	assert_eq!(my_vec.x, 0.);
	assert_eq!(my_vec.y, 1.);
}

#[test]
fn sub_scalar() {	
	let mut my_vec: Vec2 = Vec2::new(2., 3.) - 2.;
	
	assert_eq!(my_vec.x, 0.);
	assert_eq!(my_vec.y, 1.);
	
	my_vec -= 2.;
	
	assert_eq!(my_vec.x, -2.);
	assert_eq!(my_vec.y, -1.);
}

#[test]
fn sub_vec() {	
	let mut my_vec: Vec2 = Vec2::new(2., 3.) - Vec2::new(2., 2.);
	
	assert_eq!(my_vec.x, 0.);
	assert_eq!(my_vec.y, 1.);
	
	my_vec -= Vec2::new(2., 2.);
	
	assert_eq!(my_vec.x, -2.);
	assert_eq!(my_vec.y, -1.);
}

#[test]
fn sub_vec_ref() {	
	let mut my_vec: Vec2 = Vec2::new(2., 3.);
	let my_vec2: Vec2 = Vec2::new(2., 2.);
	let my_vec3: Vec2 = Vec2::new(-2., -2.);
	 
	let result: Vec2 = &my_vec - &my_vec2;
	
	assert_eq!(result.x, 0.);
	assert_eq!(result.y, 1.);
	
	my_vec -= &my_vec3;
	
	assert_eq!(my_vec.x, 4.);
	assert_eq!(my_vec.y, 5.);
}

#[test]
fn to_string() {	
	let my_vec: Vec2 = Vec2::new(2., 3.);
	let vec_string: String = my_vec.to_string();
	assert_eq!(vec_string, "x:2 y:3");
}

#[test]
fn normalize() {	
	let mut my_vec: Vec2 = Vec2::new(2., 3.);
	my_vec.normalize();
	
	assert_eq!(my_vec.x, 0.5547002);
	assert_eq!(my_vec.y, 0.8320503);
	
	assert_eq!(my_vec.len(), 1.);
}

#[test]
fn normalize_to() {	
	let my_vec: Vec2 = Vec2::new(2., 3.);
	let mut my_vec2 = Vec2::new_empty();
	
	my_vec.normalize_to(&mut my_vec2);
	
	assert_eq!(my_vec2.x, 0.5547002);
	assert_eq!(my_vec2.y, 0.8320503);
	
	assert_eq!(my_vec2.len(), 1.);
}

#[test]
fn angle() {	
	let my_vec: Vec2 = Vec2::new(0., 1.);
	let my_vec2: Vec2 = Vec2::new(1., 0.);
	
	let angle_rad: f32 = my_vec.angle(&my_vec2);
	let angle_degree: f32 = angle_rad * (180. / PI);
	
	assert_eq!(angle_degree, 90.0);
}