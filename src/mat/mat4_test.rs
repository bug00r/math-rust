use mat::mat4::*;

#[test]
fn creation() {	
	let mut mat: Mat4 = Mat4::new(1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.);
	
	assert_eq!(mat._11, 1.);
	assert_eq!(mat._12, 1.);
	assert_eq!(mat._13, 1.);
	assert_eq!(mat._14, 1.);
	assert_eq!(mat._21, 1.);
	assert_eq!(mat._22, 1.);
	assert_eq!(mat._23, 1.);
	assert_eq!(mat._24, 1.);
	assert_eq!(mat._31, 1.);
	assert_eq!(mat._32, 1.);
	assert_eq!(mat._33, 1.);
	assert_eq!(mat._34, 1.);
	assert_eq!(mat._41, 1.);
	assert_eq!(mat._42, 1.);
	assert_eq!(mat._43, 1.);
	assert_eq!(mat._44, 1.);
	
	mat.set(2., 2., 2., 2., 2., 2., 2., 2., 2., 2., 2., 2., 2., 2., 2., 2.);
	
	assert_eq!(mat._11, 2.);
	assert_eq!(mat._12, 2.);
	assert_eq!(mat._13, 2.);
	assert_eq!(mat._14, 2.);
	assert_eq!(mat._21, 2.);
	assert_eq!(mat._22, 2.);
	assert_eq!(mat._23, 2.);
	assert_eq!(mat._24, 2.);
	assert_eq!(mat._31, 2.);
	assert_eq!(mat._32, 2.);
	assert_eq!(mat._33, 2.);
	assert_eq!(mat._34, 2.);
	assert_eq!(mat._41, 2.);
	assert_eq!(mat._42, 2.);
	assert_eq!(mat._43, 2.);
	assert_eq!(mat._44, 2.);
	
	let mat3: Mat4 = Mat4::new_empty();
	
	mat.set_from(&mat3);
	
	assert_eq!(mat._11, 0.);
	assert_eq!(mat._12, 0.);
	assert_eq!(mat._13, 0.);
	assert_eq!(mat._14, 0.);
	assert_eq!(mat._21, 0.);
	assert_eq!(mat._22, 0.);
	assert_eq!(mat._23, 0.);
	assert_eq!(mat._24, 0.);
	assert_eq!(mat._31, 0.);
	assert_eq!(mat._32, 0.);
	assert_eq!(mat._33, 0.);
	assert_eq!(mat._34, 0.);
	assert_eq!(mat._41, 0.);
	assert_eq!(mat._42, 0.);
	assert_eq!(mat._43, 0.);
	assert_eq!(mat._44, 0.);
	
}

#[test]
fn determinant() {	
	let mat: Mat4 = Mat4::new(5., 0., 3., -1., 3., 0. , 0., 4., -1., 2., 4., -2., 1., 0., 0., 5.);
	
	assert_eq!(mat.det(), 66.);	
}

#[test]
fn equals() {	
	let mat: Mat4 = Mat4::new(1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.);
	let mat2: Mat4 = Mat4::new(1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.);
	
	assert_eq!(mat == mat, true);	
	assert_eq!(mat != mat, false);	
	assert_eq!(mat2 == mat, true);	
	assert_eq!(mat2 != mat, false);	

	let mat3: Mat4 = Mat4::new(1., 1., 1., 1., 1., 3., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.);
	
	assert_eq!(mat3 == mat, false);	
	assert_eq!(mat3 != mat, true);
	
}

#[test]
fn transponse() {	
	let mut mat: Mat4 = Mat4::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.);
	let mat2: Mat4 = Mat4::new( 1., 5., 9., 13., 2., 6. , 10., 14., 3., 7., 11., 15., 4., 8., 12., 16.);
	let mut mat3: Mat4 = Mat4::new_empty();
	
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
	let mat: Mat4 = Mat4::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.) + 
					Mat4::new(0., 0., 0., 0.,0., 0., 0., 0., 0., 0., 0.,0., 0., 0., 0., 0.);
	let mut mat2: Mat4 = Mat4::new(1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1., 1.);
	let matc: Mat4 = Mat4::new(-1., -1., -1., -1., -1., -1., -1., -1., -1., -1., -1., -1., -1., -1., -1., -1.);
	
	let mat3: Mat4 = &mat + &mat2;
	
	let result: Mat4 = Mat4::new(2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17.);
	assert_eq!(mat3 == result, true);	
	
	mat2 += mat3;
	
	let result2: Mat4 = Mat4::new(3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18.);
	assert_eq!(mat2 == result2, true);
	
	mat2 += &matc;
	
	let result3: Mat4 = Mat4::new(2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17.);
	assert_eq!(mat2 == result3, true);
}


#[test]
fn add_skalar() {	
	let mut mat: Mat4 = Mat4::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.) + 0.;
	
	mat = mat + 2.;
	
	let result2: Mat4 = Mat4::new(3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16., 17., 18.);
	assert_eq!(mat == result2, true);	
	
	mat += -2.;
	
	let result3: Mat4 = Mat4::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.);
	assert_eq!(mat == result3, true);
	
	
}


#[test]
fn sub() {	
	let mat: Mat4 = Mat4::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.) - 
					Mat4::new(0., 0., 0., 0.,0., 0., 0., 0., 0., 0., 0.,0., 0., 0., 0., 0.);
	let mat2: Mat4 = Mat4::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.);
	let matc: Mat4 = Mat4::new(-1., -1., -1., -1., -1., -1., -1., -1., -1., -1., -1., -1., -1., -1., -1., -1.);
	
	let mut mat3: Mat4 = &mat - &mat2;
	
	let result: Mat4 = Mat4::new(0., 0., 0., 0.,0., 0., 0., 0., 0., 0., 0.,0., 0., 0., 0., 0.);
	assert_eq!(mat3 == result, true);	
	
	mat3 -= &mat;
	
	let result2: Mat4 = Mat4::new(-1., -2., -3., -4., -5., -6., -7., -8., -9., -10., -11., -12., -13., -14., -15., -16.);
	assert_eq!(mat3 == result2, true);
	
	mat3 -= matc;
	
	let result3: Mat4 = Mat4::new(0., -1., -2., -3., -4., -5., -6., -7., -8., -9., -10., -11., -12., -13., -14., -15.);
	assert_eq!(mat3 == result3, true);
}


#[test]
fn sub_skalar() {	
	let mut mat: Mat4 = Mat4::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.) - 0.;
	
	mat = mat - 2.;
	
	let result2: Mat4 = Mat4::new(-1., 0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14.);
	assert_eq!(mat == result2, true);	
	
	mat -= -2.;
	
	let result3: Mat4 = Mat4::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.);
	assert_eq!(mat == result3, true);
}

#[test]
fn mul() {	
	let mat: Mat4 = Mat4::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.) * 
					Mat4::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.);
	let mat2: Mat4 = Mat4::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.);
	let mat3: Mat4 = Mat4::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.);
	let result = Mat4::new(90., 100.,110., 120., 202., 228., 254. , 280., 314., 356., 398., 440., 426., 484., 542., 600.);
	
	assert_eq!(mat == result, true);	
	
	let mat4: Mat4 = &mat2 * &mat3;
	
	assert_eq!(mat4 == result, true);	
	
	let mut mat5: Mat4 = mat2.clone();
	mat5 *= mat2;
	
	assert_eq!(mat5 == result, true);
	
	let mut mat6: Mat4 = mat3.clone();
	mat6 *= &mat3;
	
	assert_eq!(mat6 == result, true);
}

#[test]
fn mul_skalar() {	
	let mut mat: Mat4 = Mat4::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.) * 1.;
	
	mat = mat * 2.;
	
	let result2: Mat4 = Mat4::new(2., 4., 6., 8., 10., 12., 14., 16., 18., 20., 22., 24., 26., 28., 30., 32.);
	assert_eq!(mat == result2, true);	
	
	mat *= 0.5;
	
	let result3: Mat4 = Mat4::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.);
	assert_eq!(mat == result3, true);
}

#[test]
fn inverse() {	
	let mut inv1: Mat4 = Mat4::new(1., 2., 3., 4., -1., 2., 0., 5., 0., 4., -2., 6., 2., 4., 0., 6. );
	let inv2: Mat4 = Mat4::new(1., 2., 3., 4., -1., 2., 0., 5., 0., 4., -2., 6., 2., 4., 0., 6. );
	let unity: Mat4 = Mat4::new(1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.);
	
	inv1.invert();
	
	inv1 *= &inv2;
	assert_eq!(inv1 == unity, true);
	
	let mut inv3: Mat4 = Mat4::new_empty();
	inv2.invert_to(&mut inv3);
	
	inv3 *= &inv2;
	assert_eq!(inv3 == unity, true);
	
}

#[test]
fn copy_to() {	
	let inv1: Mat4 = Mat4::new(1., 2., 3., 4., 5., 6., 7., 8., 9., 10., 11., 12., 13., 14., 15., 16.);
	let mut inv2: Mat4 = Mat4::new_empty();

	inv1.copy_to(&mut inv2);
	
	assert_eq!(&inv1 as *const _ == &inv2 as *const _, false);
	assert_eq!(&inv1 as *const _ != &inv2 as *const _, true);
	assert_eq!(inv1 == inv2, true);	
}
