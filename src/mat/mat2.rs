use std::ops::*;

#[derive(Clone)]
pub struct Mat2 {
	pub _11: f32,
	pub _12: f32,
	pub _21: f32,
	pub _22: f32,
}

impl PartialEq for Mat2 {
	fn eq(&self, other: &Mat2) -> bool {
		(self as *const _ == other as *const _) ||  (self._11 == other._11 && self._12 == other._12 && self._21 == other._21 && self._22 == other._22)
	}
	fn ne(&self, other: &Mat2) -> bool {
		(self as *const _ != other as *const _) && (self._11 != other._11 || self._12 != other._12 || self._21 != other._21 || self._22 != other._22)
	}
}
impl Eq for Mat2 {}

impl ToString for Mat2 {
	fn to_string(&self) -> String {
		format!("\n{}\t{}\n{}\t{}\n", self._11, self._12, self._21, self._22)
	}
}

impl Add<f32> for Mat2 {
	type Output = Mat2;

	fn add(self, rhs: f32) -> Mat2 {
		Mat2 { _11: self._11 + rhs, _12: self._12 + rhs, _21: self._21 + rhs, _22: self._22 + rhs }
    }
}

impl Add<Mat2> for Mat2 {
	type Output = Mat2;
	
	fn add(self, rhs: Mat2) -> Mat2 {
		Mat2 { _11: self._11 + rhs._11, _12: self._12 + rhs._12, _21: self._21 + rhs._21, _22: self._22 + rhs._22 }
    }
}

impl<'a> Add<&'a Mat2> for &'a Mat2 {
	type Output = Mat2;
	
	fn add(self, rhs: &'a Mat2) -> Mat2 {
		Mat2 { _11: self._11 + rhs._11, _12: self._12 + rhs._12, _21: self._21 + rhs._21, _22: self._22 + rhs._22 }
    }
}

impl AddAssign<f32> for Mat2 {
	fn add_assign(&mut self, rhs: f32){
		self._11 += rhs;
		self._12 += rhs;
		self._21 += rhs;
		self._22 += rhs;
    }
}

impl AddAssign<Mat2> for Mat2 {
	
	fn add_assign(&mut self, rhs: Mat2){
		self._11 += rhs._11;
		self._12 += rhs._12;
		self._21 += rhs._21;
		self._22 += rhs._22;
    }
}

impl<'a> AddAssign<&'a Mat2> for Mat2 {
	
	fn add_assign(&mut self, rhs: &'a Mat2){
		self._11 += rhs._11;
		self._12 += rhs._12;
		self._21 += rhs._21;
		self._22 += rhs._22;
    }
}

impl Sub<f32> for Mat2 {
	type Output = Mat2;

	fn sub(self, rhs: f32) -> Mat2 {
		Mat2 { _11: self._11 - rhs, _12: self._12 - rhs, _21: self._21 - rhs, _22: self._22 - rhs }
    }
}

impl Sub<Mat2> for Mat2 {
	type Output = Mat2;
	
	fn sub(self, rhs: Mat2) -> Mat2 {
		Mat2 { _11: self._11 - rhs._11, _12: self._12 - rhs._12, _21: self._21 - rhs._21, _22: self._22 - rhs._22 }
    }
}

impl<'a> Sub<&'a Mat2> for &'a Mat2 {
	type Output = Mat2;
	
	fn sub(self, rhs: &'a Mat2) -> Mat2 {
		Mat2 { _11: self._11 - rhs._11, _12: self._12 - rhs._12, _21: self._21 - rhs._21, _22: self._22 - rhs._22 }
    }
}

impl SubAssign<f32> for Mat2 {
	fn sub_assign(&mut self, rhs: f32){
		self._11 -= rhs;
		self._12 -= rhs;
		self._21 -= rhs;
		self._22 -= rhs;
    }
}

impl SubAssign<Mat2> for Mat2 {
	
	fn sub_assign(&mut self, rhs: Mat2){
		self._11 -= rhs._11;
		self._12 -= rhs._12;
		self._21 -= rhs._21;
		self._22 -= rhs._22;
    }
}

impl<'a> SubAssign<&'a Mat2> for Mat2 {
	
	fn sub_assign(&mut self, rhs: &'a Mat2){
		self._11 -= rhs._11;
		self._12 -= rhs._12;
		self._21 -= rhs._21;
		self._22 -= rhs._22;
    }
}

impl Mul<f32> for Mat2 {
	type Output = Mat2;

	fn mul(self, rhs: f32) -> Mat2 {
		Mat2 { _11: self._11 * rhs, _12: self._12 * rhs, _21: self._21 * rhs, _22: self._22 * rhs }
    }
}

impl Mul<Mat2> for Mat2 {
	type Output = Mat2;
	
	fn mul(self, rhs: Mat2) -> Mat2 {
		Mat2 {
			_11: (self._11 * rhs._11) + (self._12 * rhs._21),
			_12: (self._11 * rhs._12) + (self._12 * rhs._22),
			_21: (self._21 * rhs._11) + (self._22 * rhs._21),
			_22: (self._21 * rhs._12) + (self._22 * rhs._22)
		}
    }
}

impl<'a> Mul<&'a Mat2> for &'a Mat2 {
	type Output = Mat2;
	
	fn mul(self, rhs: &'a Mat2) -> Mat2 {
		Mat2 {
			_11: (self._11 * rhs._11) + (self._12 * rhs._21),
			_12: (self._11 * rhs._12) + (self._12 * rhs._22),
			_21: (self._21 * rhs._11) + (self._22 * rhs._21),
			_22: (self._21 * rhs._12) + (self._22 * rhs._22)
		}
    }
}

impl MulAssign<f32> for Mat2 {
	fn mul_assign(&mut self, rhs: f32){
		self._11 *= rhs;
		self._12 *= rhs;
		self._21 *= rhs;
		self._22 *= rhs;
    }
}

impl MulAssign<Mat2> for Mat2 {
	
	fn mul_assign(&mut self, rhs: Mat2){
		let temp: Mat2 = Mat2 {
			_11: (self._11 * rhs._11) + (self._12 * rhs._21),
			_12: (self._11 * rhs._12) + (self._12 * rhs._22),
			_21: (self._21 * rhs._11) + (self._22 * rhs._21),
			_22: (self._21 * rhs._12) + (self._22 * rhs._22)
		};
		self._11 = temp._11;
		self._12 = temp._12;
		self._21 = temp._21;
		self._22 = temp._22;
    }
}

impl<'a> MulAssign<&'a Mat2> for Mat2 {
	
	fn mul_assign(&mut self, rhs: &'a Mat2){
		let temp: Mat2 = Mat2 {
			_11: (self._11 * rhs._11) + (self._12 * rhs._21),
			_12: (self._11 * rhs._12) + (self._12 * rhs._22),
			_21: (self._21 * rhs._11) + (self._22 * rhs._21),
			_22: (self._21 * rhs._12) + (self._22 * rhs._22)
		};
		self._11 = temp._11;
		self._12 = temp._12;
		self._21 = temp._21;
		self._22 = temp._22;
    }
}
impl Mat2 {

	pub fn new( _11: f32, _12: f32, _21: f32, _22: f32) -> Mat2 {
		Mat2 { _11, _12, _21, _22 }
	}

	pub fn new_empty() -> Mat2 {
		Mat2 { _11: 0., _12: 0., _21: 0., _22: 0. }
	}
	
	pub fn set(&mut self, _11: f32, _12: f32, _21: f32, _22: f32) {
		self._11 = _11;
		self._12 = _12;
		self._21 = _21;
		self._22 = _22;
	}
	
	pub fn set_from(&mut self, m2: &Mat2) {
		self._11 = m2._11;
		self._12 = m2._12;
		self._21 = m2._21;
		self._22 = m2._22;
	}
	
	pub fn copy_to(&self, m2: &mut Mat2) {
		m2._11 = self._11;
		m2._12 = self._12;
		m2._21 = self._21;
		m2._22 = self._22;
	}
	
	pub fn det(&self) -> f32 {
		(self._11*self._22) - (self._12*self._21) 
	}
	
	pub fn transponse(&mut self) {
		let tmp = self._21;
		self._21 = self._12;
		self._12 = tmp;
	}
	
	pub fn transponse_to(&self, m2: &mut Mat2) {
		m2._11 = self._11;
		m2._22 = self._22;
		m2._21 = self._12;
		m2._12 = self._21;
	}
	
	pub fn invert(&mut self) {
		let mut cof: Mat2 = Mat2 { _11: self._22 , _12: -self._21, _21: -self._12, _22: self._11 };
		cof.transponse();
		cof *= 1. / self.det();
		self.set_from(&cof);
	}
	
	pub fn invert_to(&self, m2: &mut Mat2) {
		m2.set( self._22, -self._21, -self._12, self._11);
		m2.transponse();
		*m2 *= 1. / self.det();
	}
}
