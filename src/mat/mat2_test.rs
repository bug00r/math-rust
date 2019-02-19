use mat::mat2::*;

#[test]
fn creation() {	
	let mut mat: Mat2 = Mat2::new(2., 3., 4., 5.);
	
	assert_eq!(mat._11, 2.);
	assert_eq!(mat._12, 3.);
	assert_eq!(mat._21, 4.);
	assert_eq!(mat._22, 5.);
	
	mat.set(1., 2., 3., 4.);
	
	assert_eq!(mat._11, 1.);
	assert_eq!(mat._12, 2.);
	assert_eq!(mat._21, 3.);
	assert_eq!(mat._22, 4.);
	
	let mat2: Mat2 = Mat2::new_empty();
	
	mat.set_from(&mat2);
	
	assert_eq!(mat._11, 0.);
	assert_eq!(mat._12, 0.);
	assert_eq!(mat._21, 0.);
	assert_eq!(mat._22, 0.);
	
}

#[test]
fn determinant() {	
	let mat: Mat2 = Mat2::new(1., 2., 3., 4.);
	
	assert_eq!(mat.det(), -2.);	
}

#[test]
fn equals() {	
	let mat: Mat2 = Mat2::new(1., 2., 3., 4.);
	let mat2: Mat2 = Mat2::new(1., 2., 3., 4.);
	
	assert_eq!(mat == mat, true);	
	assert_eq!(mat != mat, false);	
	assert_eq!(mat2 == mat, true);	
	assert_eq!(mat2 != mat, false);	

	let mat3: Mat2 = Mat2::new(0., 2., 3., 4.);
	
	assert_eq!(mat3 == mat, false);	
	assert_eq!(mat3 != mat, true);
	
}

/*
		t trans_ = { 1.f, 2.f, 3.f, 4.f};
		t trans_result = { 1.f, 3.f, 2.f, 4.f };
*/
#[test]
fn transponse() {	
	let mut mat: Mat2 = Mat2::new(1., 2., 3., 4.);
	let mat2: Mat2 = Mat2::new(1., 3., 2., 4.);
	let mut mat3: Mat2 = Mat2::new(1., 3., 2., 4.);
	
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
	let mat: Mat2 = Mat2::new(1., 2., 3., 4.) + Mat2::new(0., 0., 0., 0.);
	let mat2: Mat2 = Mat2::new(1., 2., 3., 4.);
	
	let mut mat3: Mat2 = &mat + &mat2;
	
	let result: Mat2 = Mat2::new(2., 4., 6., 8.);
	assert_eq!(mat3 == result, true);	
	
	mat3 += mat2;
	
	let result2: Mat2 = Mat2::new(3., 6., 9., 12.);
	assert_eq!(mat3 == result2, true);
	
	mat3 += &mat;
	
	let result3: Mat2 = Mat2::new(4., 8., 12., 16.);
	assert_eq!(mat3 == result3, true);
}

#[test]
fn add_skalar() {	
	let mut mat: Mat2 = Mat2::new(1., 2., 3., 4.) + Mat2::new(0., 0., 0., 0.);
	
	mat = mat + 2.;
	
	let result2: Mat2 = Mat2::new(3., 4., 5., 6.);
	assert_eq!(mat == result2, true);	
	
	mat += 2.;
	
	let result3: Mat2 = Mat2::new(5., 6., 7., 8.);
	assert_eq!(mat == result3, true);
	
	
}

#[test]
fn sub() {	
	let mat: Mat2 = Mat2::new(1., 2., 3., 4.) - Mat2::new(0., 0., 0., 0.);
	let mat2: Mat2 = Mat2::new(1., 2., 3., 4.);
	
	let mut mat3: Mat2 = &mat - &mat2;
	
	let result: Mat2 = Mat2::new(0., 0., 0., 0.);
	assert_eq!(mat3 == result, true);	
	
	mat3 -= mat2;
	
	let result2: Mat2 = Mat2::new(-1., -2., -3., -4.);
	assert_eq!(mat3 == result2, true);
	
	mat3 -= &mat;
	
	let result3: Mat2 = Mat2::new(-2., -4., -6., -8.);
	assert_eq!(mat3 == result3, true);
}

#[test]
fn sub_skalar() {	
	let mut mat: Mat2 = Mat2::new(1., 2., 3., 4.) - 0.;
	
	mat = mat - 2.;
	
	let result2: Mat2 = Mat2::new(-1., 0., 1., 2.);
	assert_eq!(mat == result2, true);	
	
	mat -= -2.;
	
	let result3: Mat2 = Mat2::new(1., 2., 3., 4.);
	assert_eq!(mat == result3, true);
}

/*
	t mul_ = { 1.f, 2.f, 3.f, 4.f};
		t mul1 = { 1.f, 2.f, 3.f, 4.f};
		t result ={ 7.f, 10.f, 15.f, 22.f};
*/

#[test]
fn mul() {	
	let mat: Mat2 = Mat2::new(1., 2., 3., 4.) * Mat2::new(1., 2., 3., 4.);
	let mat2: Mat2 = Mat2::new(1., 2., 3., 4.);
	let mat3: Mat2 = Mat2::new(1., 2., 3., 4.);
	let result = Mat2::new(7., 10., 15., 22.);
	
	assert_eq!(mat == result, true);	
	
	let mat4: Mat2 = &mat2 * &mat3;
	
	assert_eq!(mat4 == result, true);	
	
	let mut mat5: Mat2 = mat2.clone();
	mat5 *= mat2;
	
	assert_eq!(mat5 == result, true);
	
	let mut mat6: Mat2 = mat3.clone();
	mat6 *= &mat3;
	
	assert_eq!(mat6 == result, true);
}

#[test]
fn mul_skalar() {	
	let mut mat: Mat2 = Mat2::new(1., 2., 3., 4.) * 1.;
	
	mat = mat * 2.;
	
	let result2: Mat2 = Mat2::new(2., 4., 6., 8.);
	assert_eq!(mat == result2, true);	
	
	mat *= 2.;
	
	let result3: Mat2 = Mat2::new(4., 8., 12., 16.);
	assert_eq!(mat == result3, true);
}

#[test]
fn inverse() {	
	let mut inv1: Mat2 = Mat2::new(1., 2., 3., 4.);
	let inv2: Mat2 = Mat2::new(1., 2., 3., 4.);
	let unity: Mat2 = Mat2::new(1., 0., 0., 1.);
	
	inv1.invert();
	
	inv1 *= &inv2;
	assert_eq!(inv1 == unity, true);
	
	let mut inv3: Mat2 = Mat2::new_empty();
	inv2.invert_to(&mut inv3);
	
	inv3 *= &inv2;
	assert_eq!(inv3 == unity, true);
	
}

#[test]
fn copy_to() {	
	let inv1: Mat2 = Mat2::new(1., 2., 3., 4.);
	let mut inv2: Mat2 = Mat2::new_empty();

	inv1.copy_to(&mut inv2);
	
	assert_eq!(&inv1 as *const _ == &inv2 as *const _, false);
	assert_eq!(&inv1 as *const _ != &inv2 as *const _, true);
	assert_eq!(inv1 == inv2, true);	
}
