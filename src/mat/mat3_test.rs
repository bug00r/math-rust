use mat::mat3::*;

#[test]
fn creation() {	
	let mut mat: Mat3 = Mat3::new(1., 1., 1., 1., 1., 1., 1., 1., 1.);
	
	assert_eq!(mat._11, 1.);
	assert_eq!(mat._12, 1.);
	assert_eq!(mat._13, 1.);
	assert_eq!(mat._21, 1.);
	assert_eq!(mat._22, 1.);
	assert_eq!(mat._23, 1.);
	assert_eq!(mat._31, 1.);
	assert_eq!(mat._32, 1.);
	assert_eq!(mat._33, 1.);
	
	mat.set(2., 2., 2., 2., 2., 2., 2., 2., 2.);
	
	assert_eq!(mat._11, 2.);
	assert_eq!(mat._12, 2.);
	assert_eq!(mat._13, 2.);
	assert_eq!(mat._21, 2.);
	assert_eq!(mat._22, 2.);
	assert_eq!(mat._23, 2.);
	assert_eq!(mat._31, 2.);
	assert_eq!(mat._32, 2.);
	assert_eq!(mat._33, 2.);
	
	let mat3: Mat3 = Mat3::new_empty();
	
	mat.set_from(&mat3);
	
	assert_eq!(mat._11, 0.);
	assert_eq!(mat._12, 0.);
	assert_eq!(mat._13, 0.);
	assert_eq!(mat._21, 0.);
	assert_eq!(mat._22, 0.);
	assert_eq!(mat._23, 0.);
	assert_eq!(mat._31, 0.);
	assert_eq!(mat._32, 0.);
	assert_eq!(mat._33, 0.);
	
}

#[test]
fn determinant() {	
	let mat: Mat3 = Mat3::new(11., 2., 3., 4., 5., 6., 7., 8., 9.);
	
	assert_eq!(mat.det(), -30.);	
}

#[test]
fn equals() {	
	let mat: Mat3 = Mat3::new(1., 1., 1., 1., 1., 1., 1., 1., 1.);
	let mat2: Mat3 = Mat3::new(1., 1., 1., 1., 1., 1., 1., 1., 1.);
	
	assert_eq!(mat == mat, true);	
	assert_eq!(mat != mat, false);	
	assert_eq!(mat2 == mat, true);	
	assert_eq!(mat2 != mat, false);	

	let mat3: Mat3 = Mat3::new(1., 1., 2., 1., 3., 1., 4., 1., 1.);
	
	assert_eq!(mat3 == mat, false);	
	assert_eq!(mat3 != mat, true);
	
}

#[test]
fn transponse() {	
	let mut mat: Mat3 = Mat3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.);
	let mat2: Mat3 = Mat3::new( 1., 4., 7., 2., 5., 8., 3., 6., 9.);
	let mut mat3: Mat3 = Mat3::new_empty();
	
	mat.transponse_to(&mut mat3);
	
	assert_eq!(mat != mat3, true);
	assert_eq!(mat == mat3, false);
	assert_eq!(mat2 != mat3, false);
	assert_eq!(mat2 == mat3, true);
	
	mat.transponse();
	
	assert_eq!(mat == mat2, true);
	
	assert_eq!(mat != mat3, false);
	assert_eq!(mat == mat3, true);
	
	mat.transponse();
	assert_eq!(mat != mat2, true);		
}

#[test]
fn add() {	
	let mat: Mat3 = Mat3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.) + Mat3::new(0., 0., 0., 0.,0., 0., 0., 0., 0.);
	let mut mat2: Mat3 = Mat3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.);
	
	let mat3: Mat3 = &mat + &mat2;
	
	let result: Mat3 = Mat3::new(2., 4., 6., 8., 10., 12., 14., 16., 18.);
	assert_eq!(mat3 == result, true);	
	
	mat2 += mat3;
	
	let result2: Mat3 = Mat3::new(3., 6., 9., 12., 15., 18., 21., 24., 27.);
	assert_eq!(mat2 == result2, true);
	
	mat2 += &mat;
	
	let result3: Mat3 = Mat3::new(4., 8., 12., 16., 20., 24., 28., 32., 36.);
	assert_eq!(mat2 == result3, true);
}


#[test]
fn add_skalar() {	
	let mut mat: Mat3 = Mat3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.) + Mat3::new(0., 0., 0., 0.,0., 0., 0., 0., 0.);
	
	mat = mat + 2.;
	
	let result2: Mat3 = Mat3::new(3., 4., 5., 6., 7., 8., 9., 10., 11.);
	assert_eq!(mat == result2, true);	
	
	mat += -2.;
	
	let result3: Mat3 = Mat3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.);
	assert_eq!(mat == result3, true);
	
	
}


#[test]
fn sub() {	
	let mat: Mat3 = Mat3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.) - Mat3::new(0., 0., 0., 0.,0., 0., 0., 0., 0.);
	let mat2: Mat3 = Mat3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.);
	
	let mut mat3: Mat3 = &mat - &mat2;
	
	let result: Mat3 = Mat3::new(0., 0., 0., 0.,0., 0., 0., 0., 0.);
	assert_eq!(mat3 == result, true);	
	
	mat3 -= &mat;
	
	let result2: Mat3 = Mat3::new(-1., -2., -3., -4., -5., -6., -7., -8., -9.);
	assert_eq!(mat3 == result2, true);
	
	mat3 -= mat;
	
	let result3: Mat3 = Mat3::new(-2., -4., -6., -8., -10., -12., -14., -16., -18.);
	assert_eq!(mat3 == result3, true);
}


#[test]
fn sub_skalar() {	
	let mut mat: Mat3 = Mat3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.) - 0.;
	
	mat = mat - 2.;
	
	let result2: Mat3 = Mat3::new(-1., 0., 1., 2., 3., 4., 5., 6., 7.);
	assert_eq!(mat == result2, true);	
	
	mat -= -2.;
	
	let result3: Mat3 = Mat3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.);
	assert_eq!(mat == result3, true);
}

#[test]
fn mul() {	
	let mat: Mat3 = Mat3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.) * Mat3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.);
	let mat2: Mat3 = Mat3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.);
	let mat3: Mat3 = Mat3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.);
	let result = Mat3::new(30., 36., 42., 66., 81., 96., 102., 126., 150.);
	
	assert_eq!(mat == result, true);	
	
	let mat4: Mat3 = &mat2 * &mat3;
	
	assert_eq!(mat4 == result, true);	
	
	let mut mat5: Mat3 = mat2.clone();
	mat5 *= mat2;
	
	assert_eq!(mat5 == result, true);
	
	let mut mat6: Mat3 = mat3.clone();
	mat6 *= &mat3;
	
	assert_eq!(mat6 == result, true);
}

#[test]
fn mul_skalar() {	
	let mut mat: Mat3 = Mat3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.) * 1.;
	
	mat = mat * 2.;
	
	let result2: Mat3 = Mat3::new(2., 4., 6., 8., 10., 12., 14., 16., 18.);
	assert_eq!(mat == result2, true);	
	
	mat *= 0.5;
	
	let result3: Mat3 = Mat3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.);
	assert_eq!(mat == result3, true);
}

#[test]
fn inverse() {	
	let mut inv1: Mat3 = Mat3::new(1., 2., 0., 2., 4., 1., 2., 1., 0.);
	let inv2: Mat3 = Mat3::new(1., 2., 0., 2., 4., 1., 2., 1., 0.);
	let unity: Mat3 = Mat3::new(1., 0., 0., 0., 1., 0., 0., 0., 1.);
	
	inv1.invert();
	
	inv1 *= &inv2;
	assert_eq!(inv1 == unity, true);
	
	let mut inv3: Mat3 = Mat3::new_empty();
	inv2.invert_to(&mut inv3);
	
	inv3 *= &inv2;
	assert_eq!(inv3 == unity, true);
	
}

#[test]
fn copy_to() {	
	let inv1: Mat3 = Mat3::new(1., 2., 3., 4., 5., 6., 7., 8., 9.);
	let mut inv2: Mat3 = Mat3::new_empty();

	inv1.copy_to(&mut inv2);
	
	assert_eq!(&inv1 as *const _ == &inv2 as *const _, false);
	assert_eq!(&inv1 as *const _ != &inv2 as *const _, true);
	assert_eq!(inv1 == inv2, true);	
}
