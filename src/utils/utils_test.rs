use utils::*;
use num_complex::*;

#[test]
fn linear_interpolation() {	
	let intres = interpolate_lin(0., -1., 0., 1., 255.);
	assert_eq!(intres, 127.5);
	let intres = interpolate_lin(1., -1., 0., 1., 255.);
	assert_eq!(intres, 255.);
	let intres = interpolate_lin(-1., -1., 0., 1., 255.);
	assert_eq!(intres, 0.);
}

#[test]
fn rad_() {	
	assert_eq!(rad(30.), 0.5235987756);
	assert_eq!(rad(45.), 0.7853981634);
	assert_eq!(rad(150.), 2.6179938780);
}

#[test]
fn rand_path_degree() {
	let mut degree: f32 = rand_path_deg(32.);
	assert_eq!(degree >= -32. && degree <= 32., true);
	
	degree = rand_path_deg(22.);
	assert_eq!(degree >= -22. && degree <= 22., true);
	
	degree = rand_path_deg(11.);
	assert_eq!(degree >= -11. && degree <= 11., true);
	
}

#[test]
fn place_of_vec3_() {
	assert_eq!(place_of_vec3(&Vec3::new(0., 0., 0.), &Vec3::new(2., 2., 0.), &Vec3::new(0.5, 1.5, 0.)) < 0., true);
	assert_eq!(place_of_vec3(&Vec3::new(0., 0., 0.), &Vec3::new(2., 2., 0.), &Vec3::new(1.5, 0.5, 0.)) > 0., true);
}

#[test]
fn inside_of_triangle() {
	let v0: Vec3 = Vec3::new(0., 0., 0.);
	let v1: Vec3 = Vec3::new(2., 2., 0.);
	let v2: Vec3 = Vec3::new(2., 0., 0.);
	let point: Vec3 = Vec3::new(1.5, 0.5, 0.);
	let point1: Vec3 = Vec3::new(1., 1., 0.);
	let point2: Vec3 = Vec3::new(5., 1., 0.);
	let point3: Vec3 = Vec3::new(1., 1., 0.);
	
	let mut bc: Barycentric = Barycentric::new();
	bc.area = place_of_vec3(&v0, &v1, &v2);
	
	is_inside_triangle(&v0, &v1, &v2, &point, &mut bc);
	assert_eq!(bc.inside, true);
	
	is_inside_triangle(&v0, &v1, &v2, &point1, &mut bc);
	assert_eq!(bc.inside, true);
	
	is_inside_triangle(&v0, &v1, &v2, &point2, &mut bc);
	assert_eq!(bc.inside, false);
	
	is_inside_triangle(&v0, &v1, &v2, &point3, &mut bc);
	assert_eq!(bc.inside, true);
}

/*
	//mat4_t mat4_trans = { 1.f, 2.f, 3.f, 0.f, 1.f, 2.f, 3.f, 0.f, 1.f, 2.f, 3.f, 0.f, 0.f, 0.f, 0.f, 1.f};
	//vec3_t vec3_trans = { 2.f, 2.f, 2.f};
	//vec3_t vec3_result_trans = { 6.f, 12.f, 18.f};
	//
	//vec3_t *transformed = transform_point_new(&mat4_trans, &vec3_trans);
	//assert(vec3_equals(transformed, &vec3_result_trans));
	//
	//free(transformed);
	//
	//transform_point(&mat4_trans, &vec3_trans);
	//assert(vec3_equals(&vec3_trans, &vec3_result_trans));
	//
	//mat4_trans = (mat4_t) { 1.f, 2.f, 3.f, 0.5f, 1.f, 2.f, 3.f, 0.5f, 1.f, 2.f, 3.f, 0.5f, 0.f, 0.f, 0.f, 1.f};
	//vec3_trans = (vec3_t) { 2.f, 2.f, 2.f};
	//vec3_result_trans = (vec3_t) { (6.f/4.), (12.f/4.), (18.f/4.)};
	//
	//transform_point(&mat4_trans, &vec3_trans);
	//assert(vec3_equals(&vec3_trans, &vec3_result_trans));
*/
#[test]
fn transform_point() {
	//Here we need a new testase.
}
