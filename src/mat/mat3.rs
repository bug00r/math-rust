use std::ops::*;
use mat::mat2::*;
use std::f32::consts::PI;

const ONEDEGRAD: f32 = PI / 180.;


#[derive(Clone)]
pub struct Mat3 {
	pub _11: f32,
	pub _12: f32,
	pub _13: f32,
	pub _21: f32,
	pub _22: f32,
	pub _23: f32,
	pub _31: f32,
	pub _32: f32,
	pub _33: f32,
}

impl PartialEq for Mat3 {
	fn eq(&self, other: &Mat3) -> bool {
		(self as *const _ == other as *const _) ||  
			(self._11 == other._11 && self._12 == other._12 && self._13 == other._13 && 
			 self._21 == other._21 && self._22 == other._22 && self._23 == other._23 &&
			 self._31 == other._31 && self._32 == other._32 && self._33 == other._33)
	}
	fn ne(&self, other: &Mat3) -> bool {
		(self as *const _ != other as *const _) && 
			(self._11 != other._11 || self._12 != other._12 || self._13 != other._13 || 
			 self._21 != other._21 || self._22 != other._22 || self._23 != other._23 ||
			 self._31 != other._31 || self._32 != other._32 || self._33 != other._33)
	}
}
impl Eq for Mat3 {}

impl ToString for Mat3 {
	fn to_string(&self) -> String {
		format!("\n{}\t{}\t{}\n{}\t{}\t{}\n{}\t{}\t{}\n", self._11, self._12, self._13, 
														  self._21, self._22, self._23,
														  self._31, self._32, self._33)
	}
}

impl Add<f32> for Mat3 {
	type Output = Mat3;

	fn add(self, rhs: f32) -> Mat3 {
		Mat3 { _11: self._11 + rhs, _12: self._12 + rhs, _13: self._13 + rhs,
			   _21: self._21 + rhs, _22: self._22 + rhs, _23: self._23 + rhs,
			   _31: self._31 + rhs, _32: self._32 + rhs, _33: self._33 + rhs}
    }
}

impl Add<Mat3> for Mat3 {
	type Output = Mat3;
	
	fn add(self, rhs: Mat3) -> Mat3 {
		Mat3 { _11: self._11 + rhs._11, _12: self._12 + rhs._12, _13: self._13 + rhs._13,
			   _21: self._21 + rhs._21, _22: self._22 + rhs._22, _23: self._23 + rhs._23,
			   _31: self._31 + rhs._31, _32: self._32 + rhs._32, _33: self._33 + rhs._33}
    }
}

impl<'a> Add<&'a Mat3> for &'a Mat3 {
	type Output = Mat3;
	
	fn add(self, rhs: &'a Mat3) -> Mat3 {
		Mat3 { _11: self._11 + rhs._11, _12: self._12 + rhs._12, _13: self._13 + rhs._13,
			   _21: self._21 + rhs._21, _22: self._22 + rhs._22, _23: self._23 + rhs._23,
			   _31: self._31 + rhs._31, _32: self._32 + rhs._32, _33: self._33 + rhs._33}
    }
}

impl AddAssign<f32> for Mat3 {
	fn add_assign(&mut self, rhs: f32){
		self._11 += rhs;
		self._12 += rhs;
		self._13 += rhs;
		self._21 += rhs;
		self._22 += rhs;
		self._23 += rhs;
		self._31 += rhs;
		self._32 += rhs;
		self._33 += rhs;
    }
}

impl AddAssign<Mat3> for Mat3 {
	
	fn add_assign(&mut self, rhs: Mat3){
		self._11 += rhs._11;
		self._12 += rhs._12;
		self._13 += rhs._13;
		self._21 += rhs._21;
		self._22 += rhs._22;
		self._23 += rhs._23;
		self._31 += rhs._31;
		self._32 += rhs._32;
		self._33 += rhs._33;
    }
}

impl<'a> AddAssign<&'a Mat3> for Mat3 {
	
	fn add_assign(&mut self, rhs: &'a Mat3){
		self._11 += rhs._11;
		self._12 += rhs._12;
		self._13 += rhs._13;
		self._21 += rhs._21;
		self._22 += rhs._22;
		self._23 += rhs._23;
		self._31 += rhs._31;
		self._32 += rhs._32;
		self._33 += rhs._33;
    }
}

impl Sub<f32> for Mat3 {
	type Output = Mat3;

	fn sub(self, rhs: f32) -> Mat3 {
		Mat3 { _11: self._11 - rhs, _12: self._12 - rhs, _13: self._13 - rhs,
			   _21: self._21 - rhs, _22: self._22 - rhs, _23: self._23 - rhs,
			   _31: self._31 - rhs, _32: self._32 - rhs, _33: self._33 - rhs}
    }
}

impl Sub<Mat3> for Mat3 {
	type Output = Mat3;
	
	fn sub(self, rhs: Mat3) -> Mat3 {
		Mat3 { _11: self._11 - rhs._11, _12: self._12 - rhs._12, _13: self._13 - rhs._13,
			   _21: self._21 - rhs._21, _22: self._22 - rhs._22, _23: self._23 - rhs._23,
			   _31: self._31 - rhs._31, _32: self._32 - rhs._32, _33: self._33 - rhs._33}
    }
}

impl<'a> Sub<&'a Mat3> for &'a Mat3 {
	type Output = Mat3;
	
	fn sub(self, rhs: &'a Mat3) -> Mat3 {
		Mat3 { _11: self._11 - rhs._11, _12: self._12 - rhs._12, _13: self._13 - rhs._13,
			   _21: self._21 - rhs._21, _22: self._22 - rhs._22, _23: self._23 - rhs._23,
			   _31: self._31 - rhs._31, _32: self._32 - rhs._32, _33: self._33 - rhs._33}
    }
}

impl SubAssign<f32> for Mat3 {
	fn sub_assign(&mut self, rhs: f32){
		self._11 -= rhs;
		self._12 -= rhs;
		self._13 -= rhs;
		self._21 -= rhs;
		self._22 -= rhs;
		self._23 -= rhs;
		self._31 -= rhs;
		self._32 -= rhs;
		self._33 -= rhs;
    }
}

impl SubAssign<Mat3> for Mat3 {
	
	fn sub_assign(&mut self, rhs: Mat3){
		self._11 -= rhs._11;
		self._12 -= rhs._12;
		self._13 -= rhs._13;
		self._21 -= rhs._21;
		self._22 -= rhs._22;
		self._23 -= rhs._23;
		self._31 -= rhs._31;
		self._32 -= rhs._32;
		self._33 -= rhs._33;
    }
}

impl<'a> SubAssign<&'a Mat3> for Mat3 {
	
	fn sub_assign(&mut self, rhs: &'a Mat3){
		self._11 -= rhs._11;
		self._12 -= rhs._12;
		self._13 -= rhs._13;
		self._21 -= rhs._21;
		self._22 -= rhs._22;
		self._23 -= rhs._23;
		self._31 -= rhs._31;
		self._32 -= rhs._32;
		self._33 -= rhs._33;
    }
}

impl Mul<f32> for Mat3 {
	type Output = Mat3;

	fn mul(self, rhs: f32) -> Mat3 {
		Mat3 { _11: self._11 * rhs, _12: self._12 * rhs, _13: self._13 * rhs,
			   _21: self._21 * rhs, _22: self._22 * rhs, _23: self._23 * rhs,
			   _31: self._31 * rhs, _32: self._32 * rhs, _33: self._33 * rhs}
    }
}

impl Mul<Mat3> for Mat3 {
	type Output = Mat3;
	
	fn mul(self, rhs: Mat3) -> Mat3 {
		Mat3 {
			_11: (self._11 * rhs._11) + (self._12 * rhs._21) + (self._13 * rhs._31),
			_12: (self._11 * rhs._12) + (self._12 * rhs._22) + (self._13 * rhs._32),
			_13: (self._11 * rhs._13) + (self._12 * rhs._23) + (self._13 * rhs._33),
			_21: (self._21 * rhs._11) + (self._22 * rhs._21) + (self._23 * rhs._31),
			_22: (self._21 * rhs._12) + (self._22 * rhs._22) + (self._23 * rhs._32),
			_23: (self._21 * rhs._13) + (self._22 * rhs._23) + (self._23 * rhs._33),
			_31: (self._31 * rhs._11) + (self._32 * rhs._21) + (self._33 * rhs._31),
			_32: (self._31 * rhs._12) + (self._32 * rhs._22) + (self._33 * rhs._32),
			_33: (self._31 * rhs._13) + (self._32 * rhs._23) + (self._33 * rhs._33)
		}
    }
}

impl<'a> Mul<&'a Mat3> for &'a Mat3 {
	type Output = Mat3;
	
	fn mul(self, rhs: &'a Mat3) -> Mat3 {
		Mat3 {
			_11: (self._11 * rhs._11) + (self._12 * rhs._21) + (self._13 * rhs._31),
			_12: (self._11 * rhs._12) + (self._12 * rhs._22) + (self._13 * rhs._32),
			_13: (self._11 * rhs._13) + (self._12 * rhs._23) + (self._13 * rhs._33),
			_21: (self._21 * rhs._11) + (self._22 * rhs._21) + (self._23 * rhs._31),
			_22: (self._21 * rhs._12) + (self._22 * rhs._22) + (self._23 * rhs._32),
			_23: (self._21 * rhs._13) + (self._22 * rhs._23) + (self._23 * rhs._33),
			_31: (self._31 * rhs._11) + (self._32 * rhs._21) + (self._33 * rhs._31),
			_32: (self._31 * rhs._12) + (self._32 * rhs._22) + (self._33 * rhs._32),
			_33: (self._31 * rhs._13) + (self._32 * rhs._23) + (self._33 * rhs._33)
		}
    }
}

impl MulAssign<f32> for Mat3 {
	fn mul_assign(&mut self, rhs: f32){
		self._11 *= rhs;
		self._12 *= rhs;
		self._13 *= rhs;
		self._21 *= rhs;
		self._22 *= rhs;
		self._23 *= rhs;
		self._31 *= rhs;
		self._32 *= rhs;
		self._33 *= rhs;
    }
}

impl MulAssign<Mat3> for Mat3 {
	
	fn mul_assign(&mut self, rhs: Mat3){
		let temp: Mat3 = Mat3 {
			_11: (self._11 * rhs._11) + (self._12 * rhs._21) + (self._13 * rhs._31),
			_12: (self._11 * rhs._12) + (self._12 * rhs._22) + (self._13 * rhs._32),
			_13: (self._11 * rhs._13) + (self._12 * rhs._23) + (self._13 * rhs._33),
			_21: (self._21 * rhs._11) + (self._22 * rhs._21) + (self._23 * rhs._31),
			_22: (self._21 * rhs._12) + (self._22 * rhs._22) + (self._23 * rhs._32),
			_23: (self._21 * rhs._13) + (self._22 * rhs._23) + (self._23 * rhs._33),
			_31: (self._31 * rhs._11) + (self._32 * rhs._21) + (self._33 * rhs._31),
			_32: (self._31 * rhs._12) + (self._32 * rhs._22) + (self._33 * rhs._32),
			_33: (self._31 * rhs._13) + (self._32 * rhs._23) + (self._33 * rhs._33)
		};
		self._11 = temp._11;
		self._12 = temp._12;
		self._13 = temp._13;
		self._21 = temp._21;
		self._22 = temp._22;
		self._23 = temp._23;
		self._31 = temp._31;
		self._32 = temp._32;
		self._33 = temp._33;
    }
}

impl<'a> MulAssign<&'a Mat3> for Mat3 {
	
	fn mul_assign(&mut self, rhs: &'a Mat3){
		let temp: Mat3 = Mat3 {
			_11: (self._11 * rhs._11) + (self._12 * rhs._21) + (self._13 * rhs._31),
			_12: (self._11 * rhs._12) + (self._12 * rhs._22) + (self._13 * rhs._32),
			_13: (self._11 * rhs._13) + (self._12 * rhs._23) + (self._13 * rhs._33),
			_21: (self._21 * rhs._11) + (self._22 * rhs._21) + (self._23 * rhs._31),
			_22: (self._21 * rhs._12) + (self._22 * rhs._22) + (self._23 * rhs._32),
			_23: (self._21 * rhs._13) + (self._22 * rhs._23) + (self._23 * rhs._33),
			_31: (self._31 * rhs._11) + (self._32 * rhs._21) + (self._33 * rhs._31),
			_32: (self._31 * rhs._12) + (self._32 * rhs._22) + (self._33 * rhs._32),
			_33: (self._31 * rhs._13) + (self._32 * rhs._23) + (self._33 * rhs._33)
		};
		self._11 = temp._11;
		self._12 = temp._12;
		self._13 = temp._13;
		self._21 = temp._21;
		self._22 = temp._22;
		self._23 = temp._23;
		self._31 = temp._31;
		self._32 = temp._32;
		self._33 = temp._33;
    }
}

impl Mat3 {

	pub fn new( _11: f32, _12: f32, _13: f32, 
				_21: f32, _22: f32, _23: f32,
				_31: f32, _32: f32, _33: f32) -> Mat3 {
		Mat3 { _11, _12, _13, _21, _22, _23, _31, _32, _33 }
	}

	pub fn new_empty() -> Mat3 {
		Mat3 { _11: 0., _12: 0., _13: 0., _21: 0., _22: 0., _23: 0., _31: 0., _32: 0., _33: 0. }
	}
	
	pub fn new_rotx(degree: f32) -> Mat3 {
		let radian = ONEDEGRAD * degree;
		let sinus = radian.sin();
		let cosinus = radian.cos();
		let mut rotxmat: Mat3 = Mat3::new_empty();
		rotxmat._11 = 1.;
		rotxmat._22 = cosinus;
		rotxmat._23 = -sinus;
		rotxmat._32 = sinus;
		rotxmat._33 = cosinus;
		rotxmat
	}
	
	pub fn new_roty(degree: f32) -> Mat3 {
		let radian = ONEDEGRAD * degree;
		let sinus = radian.sin();
		let cosinus = radian.cos();
		let mut rotymat: Mat3 = Mat3::new_empty();
		rotymat._11 = cosinus;
		rotymat._13 = sinus;
		rotymat._22 = 1.;
		rotymat._31 = -sinus;
		rotymat._33 = cosinus;
		rotymat
	}
	
	pub fn new_rotz(degree: f32) -> Mat3 {
		let radian = ONEDEGRAD * degree;
		let sinus = radian.sin();
		let cosinus = radian.cos();
		let mut rotzmat: Mat3 = Mat3::new_empty();
		rotzmat._11 = cosinus;
		rotzmat._12 = -sinus;
		rotzmat._21 = sinus;
		rotzmat._22 = cosinus;
		rotzmat._33 = 1.;
		rotzmat
	}
	
	pub fn new_rot_xyz(degree_x: f32, degree_y: f32, degree_z: f32) -> Mat3 {
		Mat3::new_rotx(degree_x) * Mat3::new_roty(degree_y) * Mat3::new_rotz(degree_z)				   
	}
	
	
	pub fn to_rotx(&mut self, degree: f32) {
		let radian = ONEDEGRAD * degree;
		let sinus = radian.sin();
		let cosinus = radian.cos();
		self._11 = 1.;
		self._12 = 0.;
		self._13 = 0.;
		self._21 = 0.;
		self._22 = cosinus;
		self._23 = -sinus;
		self._31 = 0.;
		self._32 = sinus;
		self._33 = cosinus;
	}
	
	pub fn to_roty(&mut self, degree: f32) {
		let radian = ONEDEGRAD * degree;
		let sinus = radian.sin();
		let cosinus = radian.cos();
		self._11 = cosinus;
		self._12 = 0.;
		self._13 = sinus;
		self._21 = 0.;
		self._22 = 1.;
		self._23 = 0.;
		self._31 = -sinus;
		self._32 = 0.;
		self._33 = cosinus;
	}
	
	pub fn to_rotz(&mut self, degree: f32) {
		let radian = ONEDEGRAD * degree;
		let sinus = radian.sin();
		let cosinus = radian.cos();
		self._11 = cosinus;
		self._12 = -sinus;
		self._13 = 0.;
		self._21 = sinus;
		self._22 = cosinus;
		self._23 = 0.;
		self._31 = 0.;
		self._32 = 0.;
		self._33 = 1.;
	}
	
	pub fn to_rot_xyz(&mut self, degree_x: f32, degree_y: f32, degree_z: f32) {
		self.set_from(&Mat3::new_rot_xyz(degree_x, degree_y, degree_z));			   
	}
	
	pub fn set(&mut self, _11: f32, _12: f32, _13: f32, 
						  _21: f32, _22: f32, _23: f32,
						  _31: f32, _32: f32, _33: f32) {
		self._11 = _11;
		self._12 = _12;
		self._13 = _13;
		self._21 = _21;
		self._22 = _22;
		self._23 = _23;
		self._31 = _31;
		self._32 = _32;
		self._33 = _33;
	}
	
	pub fn set_from(&mut self, m2: &Mat3) {
		self._11 = m2._11;
		self._12 = m2._12;
		self._13 = m2._13;
		self._21 = m2._21;
		self._22 = m2._22;
		self._23 = m2._23;
		self._31 = m2._31;
		self._32 = m2._32;
		self._33 = m2._33;
	}
	
	pub fn copy_to(&self, m2: &mut Mat3) {
		m2._11 = self._11;
		m2._12 = self._12;
		m2._13 = self._13;
		m2._21 = self._21;
		m2._22 = self._22;
		m2._23 = self._23;
		m2._31 = self._31;
		m2._32 = self._32;
		m2._33 = self._33;
	}
	
	pub fn det(&self) -> f32 {
	   (self._11*self._22*self._33) +
	   (self._12*self._23*self._31) + 
	   (self._13*self._21*self._32) -
	   (self._13*self._22*self._31) -
	   (self._12*self._21*self._33) -
	   (self._11*self._23*self._32)
	}
	
	pub fn transponse(&mut self) {
		let mut temp: f32 = self._21;
		self._21 = self._12;
		self._12 = temp;
		temp = self._31;
		self._31 = self._13;
		self._13 = temp;
		temp = self._32;
		self._32 = self._23;
		self._23 = temp;
	}
	
	pub fn transponse_to(&self, m2: &mut Mat3) {
		m2._11 = self._11;
		m2._22 = self._22;
		m2._33 = self._33;
		m2._21 = self._12;
		m2._12 = self._21;
		m2._31 = self._13;
		m2._13 = self._31;
		m2._32 = self._23;
		m2._23 = self._32;
	}
	
	pub fn invert(&mut self) {
		let mut temp: Mat2 = Mat2::new_empty();
		let mut cof_mat3: Mat3 = Mat3::new_empty();
		
		temp.set( self._22, self._23, self._32, self._33);
		cof_mat3._11 = temp.det();
		temp.set( self._21, self._23, self._31, self._33);
		cof_mat3._12 = -temp.det();
		temp.set( self._21, self._22, self._31, self._32);
		cof_mat3._13 = temp.det();
		temp.set( self._12, self._13, self._32, self._33);
		cof_mat3._21 = -temp.det();
		temp.set( self._11, self._13, self._31, self._33);
		cof_mat3._22 = temp.det();
		temp.set( self._11, self._12, self._31, self._32);
		cof_mat3._23 = -temp.det();
		temp.set( self._12, self._13, self._22, self._23);
		cof_mat3._31 = temp.det();
		temp.set( self._11, self._13, self._21, self._23);
		cof_mat3._32 = -temp.det();
		temp.set( self._11, self._12, self._21, self._22);
		cof_mat3._33 = temp.det();
		
		cof_mat3.transponse();
		cof_mat3 *= 1. / self.det();
		self.set_from(&cof_mat3);
	}
	
	pub fn invert_to(&self, m2: &mut Mat3) {
		let mut temp: Mat2 = Mat2::new_empty();
		
		temp.set( self._22, self._23, self._32, self._33);
		m2._11 = temp.det();
		temp.set( self._21, self._23, self._31, self._33);
		m2._12 = -temp.det();
		temp.set( self._21, self._22, self._31, self._32);
		m2._13 = temp.det();
		temp.set( self._12, self._13, self._32, self._33);
		m2._21 = -temp.det();
		temp.set( self._11, self._13, self._31, self._33);
		m2._22 = temp.det();
		temp.set( self._11, self._12, self._31, self._32);
		m2._23 = -temp.det();
		temp.set( self._12, self._13, self._22, self._23);
		m2._31 = temp.det();
		temp.set( self._11, self._13, self._21, self._23);
		m2._32 = -temp.det();
		temp.set( self._11, self._12, self._21, self._22);
		m2._33 = temp.det();
		
		m2.transponse();
		*m2 *= 1. / self.det();
	}
}