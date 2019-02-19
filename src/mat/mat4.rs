use std::ops::*;
use mat::mat3::*;

#[derive(Clone)]
pub struct Mat4 {
	pub _11: f32,
	pub _12: f32,
	pub _13: f32,
	pub _14: f32,
	pub _21: f32,
	pub _22: f32,
	pub _23: f32,
	pub _24: f32,
	pub _31: f32,
	pub _32: f32,
	pub _33: f32,
	pub _34: f32,
	pub _41: f32,
	pub _42: f32,
	pub _43: f32,
	pub _44: f32,
}

impl PartialEq for Mat4 {
	fn eq(&self, other: &Mat4) -> bool {
		(self as *const _ == other as *const _) ||  
			(self._11 == other._11 && self._12 == other._12 && self._13 == other._13 && self._14 == other._14 && 
			 self._21 == other._21 && self._22 == other._22 && self._23 == other._23 && self._24 == other._24 &&
			 self._31 == other._31 && self._32 == other._32 && self._33 == other._33 && self._34 == other._34 &&
			 self._41 == other._41 && self._42 == other._42 && self._43 == other._43 && self._44 == other._44)
	}
	fn ne(&self, other: &Mat4) -> bool {
		(self as *const _ != other as *const _) && 
			(self._11 != other._11 || self._12 != other._12 || self._13 != other._13 || self._14 != other._14 || 
			 self._21 != other._21 || self._22 != other._22 || self._23 != other._23 || self._24 != other._24 ||
			 self._31 != other._31 || self._32 != other._32 || self._33 != other._33 || self._34 != other._34 ||
			 self._41 != other._41 || self._42 != other._42 || self._43 != other._43 || self._44 != other._44)
	}
}
impl Eq for Mat4 {}

impl ToString for Mat4 {
	fn to_string(&self) -> String {
		format!("\n{}\t{}\t{}\t{}\n{}\t{}\t{}\t{}\n{}\t{}\t{}\t{}\n{}\t{}\t{}\t{}\n", 
			self._11, self._12, self._13, self._14, 
			self._21, self._22, self._23, self._24,
			self._31, self._32, self._33, self._34,
			self._41, self._42, self._43, self._44)
	}
}

impl Add<f32> for Mat4 {
	type Output = Mat4;

	fn add(self, rhs: f32) -> Mat4 {
		Mat4 { _11: self._11 + rhs, _12: self._12 + rhs, _13: self._13 + rhs, _14: self._14 + rhs,
			   _21: self._21 + rhs, _22: self._22 + rhs, _23: self._23 + rhs, _24: self._24 + rhs,
			   _31: self._31 + rhs, _32: self._32 + rhs, _33: self._33 + rhs, _34: self._34 + rhs,
			   _41: self._41 + rhs, _42: self._42 + rhs, _43: self._43 + rhs, _44: self._44 + rhs}
    }
}

impl Add<Mat4> for Mat4 {
	type Output = Mat4;
	
	fn add(self, rhs: Mat4) -> Mat4 {
		Mat4 { _11: self._11 + rhs._11, _12: self._12 + rhs._12, _13: self._13 + rhs._13, _14: self._14 + rhs._14,
			   _21: self._21 + rhs._21, _22: self._22 + rhs._22, _23: self._23 + rhs._23, _24: self._24 + rhs._24,
			   _31: self._31 + rhs._31, _32: self._32 + rhs._32, _33: self._33 + rhs._33, _34: self._34 + rhs._34,
			   _41: self._41 + rhs._41, _42: self._42 + rhs._42, _43: self._43 + rhs._43, _44: self._44 + rhs._44}
    }
}

impl<'a> Add<&'a Mat4> for &'a Mat4 {
	type Output = Mat4;
	
	fn add(self, rhs: &'a Mat4) -> Mat4 {
		Mat4 { _11: self._11 + rhs._11, _12: self._12 + rhs._12, _13: self._13 + rhs._13, _14: self._14 + rhs._14,
			   _21: self._21 + rhs._21, _22: self._22 + rhs._22, _23: self._23 + rhs._23, _24: self._24 + rhs._24,
			   _31: self._31 + rhs._31, _32: self._32 + rhs._32, _33: self._33 + rhs._33, _34: self._34 + rhs._34,
			   _41: self._41 + rhs._41, _42: self._42 + rhs._42, _43: self._43 + rhs._43, _44: self._44 + rhs._44}
    }
}

impl AddAssign<f32> for Mat4 {
	fn add_assign(&mut self, rhs: f32){
		self._11 += rhs;
		self._12 += rhs;
		self._13 += rhs;
		self._14 += rhs;
		self._21 += rhs;
		self._22 += rhs;
		self._23 += rhs;
		self._24 += rhs;
		self._31 += rhs;
		self._32 += rhs;
		self._33 += rhs;
		self._34 += rhs;
		self._41 += rhs;
		self._42 += rhs;
		self._43 += rhs;
		self._44 += rhs;
    }
}

impl AddAssign<Mat4> for Mat4 {
	
	fn add_assign(&mut self, rhs: Mat4){
		self._11 += rhs._11;
		self._12 += rhs._12;
		self._13 += rhs._13;
		self._14 += rhs._14;
		self._21 += rhs._21;
		self._22 += rhs._22;
		self._23 += rhs._23;
		self._24 += rhs._24;
		self._31 += rhs._31;
		self._32 += rhs._32;
		self._33 += rhs._33;
		self._34 += rhs._34;
		self._41 += rhs._41;
		self._42 += rhs._42;
		self._43 += rhs._43;
		self._44 += rhs._44;
    }
}

impl<'a> AddAssign<&'a Mat4> for Mat4 {
	
	fn add_assign(&mut self, rhs: &'a Mat4){
		self._11 += rhs._11;
		self._12 += rhs._12;
		self._13 += rhs._13;
		self._14 += rhs._14;
		self._21 += rhs._21;
		self._22 += rhs._22;
		self._23 += rhs._23;
		self._24 += rhs._24;
		self._31 += rhs._31;
		self._32 += rhs._32;
		self._33 += rhs._33;
		self._34 += rhs._34;
		self._41 += rhs._41;
		self._42 += rhs._42;
		self._43 += rhs._43;
		self._44 += rhs._44;
    }
}

impl Sub<f32> for Mat4 {
	type Output = Mat4;

	fn sub(self, rhs: f32) -> Mat4 {
		Mat4 { _11: self._11 - rhs, _12: self._12 - rhs, _13: self._13 - rhs, _14: self._14 - rhs,
			   _21: self._21 - rhs, _22: self._22 - rhs, _23: self._23 - rhs, _24: self._24 - rhs,
			   _31: self._31 - rhs, _32: self._32 - rhs, _33: self._33 - rhs, _34: self._34 - rhs,
			   _41: self._41 - rhs, _42: self._42 - rhs, _43: self._43 - rhs, _44: self._44 - rhs}
    }
}

impl Sub<Mat4> for Mat4 {
	type Output = Mat4;
	
	fn sub(self, rhs: Mat4) -> Mat4 {
		Mat4 { _11: self._11 - rhs._11, _12: self._12 - rhs._12, _13: self._13 - rhs._13, _14: self._14 - rhs._14,
			   _21: self._21 - rhs._21, _22: self._22 - rhs._22, _23: self._23 - rhs._23, _24: self._24 - rhs._24,
			   _31: self._31 - rhs._31, _32: self._32 - rhs._32, _33: self._33 - rhs._33, _34: self._34 - rhs._34,
			   _41: self._41 - rhs._41, _42: self._42 - rhs._42, _43: self._43 - rhs._43, _44: self._44 - rhs._44}
    }
}

impl<'a> Sub<&'a Mat4> for &'a Mat4 {
	type Output = Mat4;
	
	fn sub(self, rhs: &'a Mat4) -> Mat4 {
		Mat4 { _11: self._11 - rhs._11, _12: self._12 - rhs._12, _13: self._13 - rhs._13, _14: self._14 - rhs._14,
			   _21: self._21 - rhs._21, _22: self._22 - rhs._22, _23: self._23 - rhs._23, _24: self._24 - rhs._24,
			   _31: self._31 - rhs._31, _32: self._32 - rhs._32, _33: self._33 - rhs._33, _34: self._34 - rhs._34,
			   _41: self._41 - rhs._41, _42: self._42 - rhs._42, _43: self._43 - rhs._43, _44: self._44 - rhs._44}
    }
}

impl SubAssign<f32> for Mat4 {
	fn sub_assign(&mut self, rhs: f32){
		self._11 -= rhs;
		self._12 -= rhs;
		self._13 -= rhs;
		self._14 -= rhs;
		self._21 -= rhs;
		self._22 -= rhs;
		self._23 -= rhs;
		self._24 -= rhs;
		self._31 -= rhs;
		self._32 -= rhs;
		self._33 -= rhs;
		self._34 -= rhs;
		self._41 -= rhs;
		self._42 -= rhs;
		self._43 -= rhs;
		self._44 -= rhs;
    }
}

impl SubAssign<Mat4> for Mat4 {
	
	fn sub_assign(&mut self, rhs: Mat4){
		self._11 -= rhs._11;
		self._12 -= rhs._12;
		self._13 -= rhs._13;
		self._14 -= rhs._14;
		self._21 -= rhs._21;
		self._22 -= rhs._22;
		self._23 -= rhs._23;
		self._24 -= rhs._24;
		self._31 -= rhs._31;
		self._32 -= rhs._32;
		self._33 -= rhs._33;
		self._34 -= rhs._34;
		self._41 -= rhs._41;
		self._42 -= rhs._42;
		self._43 -= rhs._43;
		self._44 -= rhs._44;
    }
}

impl<'a> SubAssign<&'a Mat4> for Mat4 {
	
	fn sub_assign(&mut self, rhs: &'a Mat4){
		self._11 -= rhs._11;
		self._12 -= rhs._12;
		self._13 -= rhs._13;
		self._14 -= rhs._14;
		self._21 -= rhs._21;
		self._22 -= rhs._22;
		self._23 -= rhs._23;
		self._24 -= rhs._24;
		self._31 -= rhs._31;
		self._32 -= rhs._32;
		self._33 -= rhs._33;
		self._34 -= rhs._34;
		self._41 -= rhs._41;
		self._42 -= rhs._42;
		self._43 -= rhs._43;
		self._44 -= rhs._44;
    }
}

impl Mul<f32> for Mat4 {
	type Output = Mat4;

	fn mul(self, rhs: f32) -> Mat4 {
		Mat4 { _11: self._11 * rhs, _12: self._12 * rhs, _13: self._13 * rhs, _14: self._14 * rhs,
			   _21: self._21 * rhs, _22: self._22 * rhs, _23: self._23 * rhs, _24: self._24 * rhs,
			   _31: self._31 * rhs, _32: self._32 * rhs, _33: self._33 * rhs, _34: self._34 * rhs,
			   _41: self._41 * rhs, _42: self._42 * rhs, _43: self._43 * rhs, _44: self._44 * rhs}
    }
}

impl Mul<Mat4> for Mat4 {
	type Output = Mat4;
	
	fn mul(self, rhs: Mat4) -> Mat4 {
		Mat4 {
			_11: (self._11 * rhs._11) + (self._12 * rhs._21) + (self._13 * rhs._31) + (self._14 * rhs._41),
			_12: (self._11 * rhs._12) + (self._12 * rhs._22) + (self._13 * rhs._32) + (self._14 * rhs._42),
			_13: (self._11 * rhs._13) + (self._12 * rhs._23) + (self._13 * rhs._33) + (self._14 * rhs._43),
			_14: (self._11 * rhs._14) + (self._12 * rhs._24) + (self._13 * rhs._34) + (self._14 * rhs._44),
			_21: (self._21 * rhs._11) + (self._22 * rhs._21) + (self._23 * rhs._31) + (self._24 * rhs._41),
			_22: (self._21 * rhs._12) + (self._22 * rhs._22) + (self._23 * rhs._32) + (self._24 * rhs._42),
			_23: (self._21 * rhs._13) + (self._22 * rhs._23) + (self._23 * rhs._33) + (self._24 * rhs._43),
			_24: (self._21 * rhs._14) + (self._22 * rhs._24) + (self._23 * rhs._34) + (self._24 * rhs._44),
			_31: (self._31 * rhs._11) + (self._32 * rhs._21) + (self._33 * rhs._31) + (self._34 * rhs._41),
			_32: (self._31 * rhs._12) + (self._32 * rhs._22) + (self._33 * rhs._32) + (self._34 * rhs._42),
			_33: (self._31 * rhs._13) + (self._32 * rhs._23) + (self._33 * rhs._33) + (self._34 * rhs._43),
			_34: (self._31 * rhs._14) + (self._32 * rhs._24) + (self._33 * rhs._34) + (self._34 * rhs._44),
			_41: (self._41 * rhs._11) + (self._42 * rhs._21) + (self._43 * rhs._31) + (self._44 * rhs._41),
			_42: (self._41 * rhs._12) + (self._42 * rhs._22) + (self._43 * rhs._32) + (self._44 * rhs._42),
			_43: (self._41 * rhs._13) + (self._42 * rhs._23) + (self._43 * rhs._33) + (self._44 * rhs._43),
			_44: (self._41 * rhs._14) + (self._42 * rhs._24) + (self._43 * rhs._34) + (self._44 * rhs._44)
		}
    }
}

impl<'a> Mul<&'a Mat4> for &'a Mat4 {
	type Output = Mat4;
	
	fn mul(self, rhs: &'a Mat4) -> Mat4 {
		Mat4 {
			_11: (self._11 * rhs._11) + (self._12 * rhs._21) + (self._13 * rhs._31) + (self._14 * rhs._41),
			_12: (self._11 * rhs._12) + (self._12 * rhs._22) + (self._13 * rhs._32) + (self._14 * rhs._42),
			_13: (self._11 * rhs._13) + (self._12 * rhs._23) + (self._13 * rhs._33) + (self._14 * rhs._43),
			_14: (self._11 * rhs._14) + (self._12 * rhs._24) + (self._13 * rhs._34) + (self._14 * rhs._44),
			_21: (self._21 * rhs._11) + (self._22 * rhs._21) + (self._23 * rhs._31) + (self._24 * rhs._41),
			_22: (self._21 * rhs._12) + (self._22 * rhs._22) + (self._23 * rhs._32) + (self._24 * rhs._42),
			_23: (self._21 * rhs._13) + (self._22 * rhs._23) + (self._23 * rhs._33) + (self._24 * rhs._43),
			_24: (self._21 * rhs._14) + (self._22 * rhs._24) + (self._23 * rhs._34) + (self._24 * rhs._44),
			_31: (self._31 * rhs._11) + (self._32 * rhs._21) + (self._33 * rhs._31) + (self._34 * rhs._41),
			_32: (self._31 * rhs._12) + (self._32 * rhs._22) + (self._33 * rhs._32) + (self._34 * rhs._42),
			_33: (self._31 * rhs._13) + (self._32 * rhs._23) + (self._33 * rhs._33) + (self._34 * rhs._43),
			_34: (self._31 * rhs._14) + (self._32 * rhs._24) + (self._33 * rhs._34) + (self._34 * rhs._44),
			_41: (self._41 * rhs._11) + (self._42 * rhs._21) + (self._43 * rhs._31) + (self._44 * rhs._41),
			_42: (self._41 * rhs._12) + (self._42 * rhs._22) + (self._43 * rhs._32) + (self._44 * rhs._42),
			_43: (self._41 * rhs._13) + (self._42 * rhs._23) + (self._43 * rhs._33) + (self._44 * rhs._43),
			_44: (self._41 * rhs._14) + (self._42 * rhs._24) + (self._43 * rhs._34) + (self._44 * rhs._44)
		}
    }
}

impl MulAssign<f32> for Mat4 {
	fn mul_assign(&mut self, rhs: f32){
		self._11 *= rhs;
		self._12 *= rhs;
		self._13 *= rhs;
		self._14 *= rhs;
		self._21 *= rhs;
		self._22 *= rhs;
		self._23 *= rhs;
		self._24 *= rhs;
		self._31 *= rhs;
		self._32 *= rhs;
		self._33 *= rhs;
		self._34 *= rhs;
		self._41 *= rhs;
		self._42 *= rhs;
		self._43 *= rhs;
		self._44 *= rhs;
    }
}

impl MulAssign<Mat4> for Mat4 {
	
	fn mul_assign(&mut self, rhs: Mat4){
		let temp: Mat4 = Mat4 {
			_11: (self._11 * rhs._11) + (self._12 * rhs._21) + (self._13 * rhs._31) + (self._14 * rhs._41),
			_12: (self._11 * rhs._12) + (self._12 * rhs._22) + (self._13 * rhs._32) + (self._14 * rhs._42),
			_13: (self._11 * rhs._13) + (self._12 * rhs._23) + (self._13 * rhs._33) + (self._14 * rhs._43),
			_14: (self._11 * rhs._14) + (self._12 * rhs._24) + (self._13 * rhs._34) + (self._14 * rhs._44),
			_21: (self._21 * rhs._11) + (self._22 * rhs._21) + (self._23 * rhs._31) + (self._24 * rhs._41),
			_22: (self._21 * rhs._12) + (self._22 * rhs._22) + (self._23 * rhs._32) + (self._24 * rhs._42),
			_23: (self._21 * rhs._13) + (self._22 * rhs._23) + (self._23 * rhs._33) + (self._24 * rhs._43),
			_24: (self._21 * rhs._14) + (self._22 * rhs._24) + (self._23 * rhs._34) + (self._24 * rhs._44),
			_31: (self._31 * rhs._11) + (self._32 * rhs._21) + (self._33 * rhs._31) + (self._34 * rhs._41),
			_32: (self._31 * rhs._12) + (self._32 * rhs._22) + (self._33 * rhs._32) + (self._34 * rhs._42),
			_33: (self._31 * rhs._13) + (self._32 * rhs._23) + (self._33 * rhs._33) + (self._34 * rhs._43),
			_34: (self._31 * rhs._14) + (self._32 * rhs._24) + (self._33 * rhs._34) + (self._34 * rhs._44),
			_41: (self._41 * rhs._11) + (self._42 * rhs._21) + (self._43 * rhs._31) + (self._44 * rhs._41),
			_42: (self._41 * rhs._12) + (self._42 * rhs._22) + (self._43 * rhs._32) + (self._44 * rhs._42),
			_43: (self._41 * rhs._13) + (self._42 * rhs._23) + (self._43 * rhs._33) + (self._44 * rhs._43),
			_44: (self._41 * rhs._14) + (self._42 * rhs._24) + (self._43 * rhs._34) + (self._44 * rhs._44)
		};
		self._11 = temp._11;
		self._12 = temp._12;
		self._13 = temp._13;
		self._14 = temp._14;
		self._21 = temp._21;
		self._22 = temp._22;
		self._23 = temp._23;
		self._24 = temp._24;
		self._31 = temp._31;
		self._32 = temp._32;
		self._33 = temp._33;
		self._34 = temp._34;
		self._41 = temp._41;
		self._42 = temp._42;
		self._43 = temp._43;
		self._44 = temp._44;
    }
}

impl<'a> MulAssign<&'a Mat4> for Mat4 {
	
	fn mul_assign(&mut self, rhs: &'a Mat4){
		let temp: Mat4 = Mat4 {
			_11: (self._11 * rhs._11) + (self._12 * rhs._21) + (self._13 * rhs._31) + (self._14 * rhs._41),
			_12: (self._11 * rhs._12) + (self._12 * rhs._22) + (self._13 * rhs._32) + (self._14 * rhs._42),
			_13: (self._11 * rhs._13) + (self._12 * rhs._23) + (self._13 * rhs._33) + (self._14 * rhs._43),
			_14: (self._11 * rhs._14) + (self._12 * rhs._24) + (self._13 * rhs._34) + (self._14 * rhs._44),
			_21: (self._21 * rhs._11) + (self._22 * rhs._21) + (self._23 * rhs._31) + (self._24 * rhs._41),
			_22: (self._21 * rhs._12) + (self._22 * rhs._22) + (self._23 * rhs._32) + (self._24 * rhs._42),
			_23: (self._21 * rhs._13) + (self._22 * rhs._23) + (self._23 * rhs._33) + (self._24 * rhs._43),
			_24: (self._21 * rhs._14) + (self._22 * rhs._24) + (self._23 * rhs._34) + (self._24 * rhs._44),
			_31: (self._31 * rhs._11) + (self._32 * rhs._21) + (self._33 * rhs._31) + (self._34 * rhs._41),
			_32: (self._31 * rhs._12) + (self._32 * rhs._22) + (self._33 * rhs._32) + (self._34 * rhs._42),
			_33: (self._31 * rhs._13) + (self._32 * rhs._23) + (self._33 * rhs._33) + (self._34 * rhs._43),
			_34: (self._31 * rhs._14) + (self._32 * rhs._24) + (self._33 * rhs._34) + (self._34 * rhs._44),
			_41: (self._41 * rhs._11) + (self._42 * rhs._21) + (self._43 * rhs._31) + (self._44 * rhs._41),
			_42: (self._41 * rhs._12) + (self._42 * rhs._22) + (self._43 * rhs._32) + (self._44 * rhs._42),
			_43: (self._41 * rhs._13) + (self._42 * rhs._23) + (self._43 * rhs._33) + (self._44 * rhs._43),
			_44: (self._41 * rhs._14) + (self._42 * rhs._24) + (self._43 * rhs._34) + (self._44 * rhs._44)
		};
		self._11 = temp._11;
		self._12 = temp._12;
		self._13 = temp._13;
		self._14 = temp._14;
		self._21 = temp._21;
		self._22 = temp._22;
		self._23 = temp._23;
		self._24 = temp._24;
		self._31 = temp._31;
		self._32 = temp._32;
		self._33 = temp._33;
		self._34 = temp._34;
		self._41 = temp._41;
		self._42 = temp._42;
		self._43 = temp._43;
		self._44 = temp._44;
    }
}

impl Mat4 {

	pub fn new( _11: f32, _12: f32, _13: f32, _14: f32, 
				_21: f32, _22: f32, _23: f32, _24: f32, 
				_31: f32, _32: f32, _33: f32, _34: f32, 
				_41: f32, _42: f32, _43: f32, _44: f32) -> Mat4 {
		Mat4 { _11, _12, _13, _14, _21, _22, _23, _24, _31, _32, _33, _34, _41, _42, _43, _44 }
	}

	pub fn new_empty() -> Mat4 {
		Mat4 { _11: 0., _12: 0., _13: 0., _14: 0., 
			   _21: 0., _22: 0., _23: 0., _24: 0., 
			   _31: 0., _32: 0., _33: 0., _34: 0., 
			   _41: 0., _42: 0., _43: 0., _44: 0. }
	}
		
	pub fn set(&mut self, _11: f32, _12: f32, _13: f32, _14: f32, 
						  _21: f32, _22: f32, _23: f32, _24: f32, 
						  _31: f32, _32: f32, _33: f32, _34: f32, 
						  _41: f32, _42: f32, _43: f32, _44: f32) {
		self._11 = _11;
		self._12 = _12;
		self._13 = _13;
		self._14 = _14;
		self._21 = _21;
		self._22 = _22;
		self._23 = _23;
		self._24 = _24;
		self._31 = _31;
		self._32 = _32;
		self._33 = _33;
		self._34 = _34;
		self._41 = _41;
		self._42 = _42;
		self._43 = _43;
		self._44 = _44;
	}
	
	pub fn set_from(&mut self, m2: &Mat4) {
		self._11 = m2._11;
		self._12 = m2._12;
		self._13 = m2._13;
		self._14 = m2._14;
		self._21 = m2._21;
		self._22 = m2._22;
		self._23 = m2._23;
		self._24 = m2._24;
		self._31 = m2._31;
		self._32 = m2._32;
		self._33 = m2._33;
		self._34 = m2._34;
		self._41 = m2._41;
		self._42 = m2._42;
		self._43 = m2._43;
		self._44 = m2._44;
	}
	
	pub fn copy_to(&self, m2: &mut Mat4) {
		m2._11 = self._11;
		m2._12 = self._12;
		m2._13 = self._13;
		m2._14 = self._14;
		m2._21 = self._21;
		m2._22 = self._22;
		m2._23 = self._23;
		m2._24 = self._24;
		m2._31 = self._31;
		m2._32 = self._32;
		m2._33 = self._33;
		m2._34 = self._34;
		m2._41 = self._41;
		m2._42 = self._42;
		m2._43 = self._43;
		m2._44 = self._44;
	}
	
	pub fn det(&self) -> f32 {
		let mut determinant: f32 = 0.;
		let mut temp: Mat3 = Mat3::new_empty();
		temp.set( self._22, self._23, self._24, 
				  self._32, self._33, self._34, 
				  self._42, self._43, self._44);
		determinant += self._11 * temp.det();
		temp.set( self._21, self._23, self._24, 
				  self._31, self._33, self._34, 
				  self._41, self._43, self._44);
		determinant += self._12 * temp.det();
		temp.set( self._21, self._22, self._24, 
				  self._31, self._32, self._34, 
				  self._41, self._42, self._44);
		determinant += self._13 * temp.det();
		temp.set( self._21, self._22, self._23, 
				  self._31, self._32, self._33, 
				  self._41, self._42, self._43);
		determinant += self._14 * temp.det();
		determinant
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
		temp = self._41;
		self._41 = self._14;
		self._14 = temp;
		temp = self._42;
		self._42 = self._24;
		self._24 = temp;
		temp = self._43;
		self._43 = self._34;
		self._34 = temp;
	}
	
	pub fn transponse_to(&self, m2: &mut Mat4) {
		m2._11 = self._11;
		m2._22 = self._22;
		m2._33 = self._33;
		m2._44 = self._44;
		m2._21 = self._12;
		m2._12 = self._21;
		m2._31 = self._13;
		m2._13 = self._31;
		m2._32 = self._23;
		m2._23 = self._32;
		m2._41 = self._14;
		m2._14 = self._41;
		m2._42 = self._24;
		m2._24 = self._42;
		m2._43 = self._34;
		m2._34 = self._43;
	}

	pub fn invert(&mut self) {
		let mut temp: Mat3 = Mat3::new_empty();
		let mut cof_mat4 = Mat4::new_empty();
		
		temp.set(self._22,self._23,self._24,self._32,self._33,self._34,self._42,self._43,self._44);
		cof_mat4._11 = temp.det();
		temp.set(self._21,self._23,self._24,self._31,self._33,self._34,self._41,self._43,self._44);
		cof_mat4._12 = -temp.det();
		temp.set(self._21,self._22,self._24,self._31,self._32,self._34,self._41,self._42,self._44);
		cof_mat4._13 = temp.det();
		temp.set(self._21,self._22,self._23,self._31,self._32,self._33,self._41,self._42,self._43);
		cof_mat4._14 = -temp.det();
		
		temp.set(self._12,self._13,self._14,self._32,self._33,self._34,self._42,self._43,self._44);
		cof_mat4._21 = -temp.det();
		temp.set(self._11,self._13,self._14,self._31,self._33,self._34,self._41,self._43,self._44);
		cof_mat4._22 = temp.det();
		temp.set(self._11,self._12,self._14,self._31,self._32,self._34,self._41,self._42,self._44);
		cof_mat4._23 = -temp.det();
		temp.set(self._11,self._12,self._13,self._31,self._32,self._33,self._41,self._42,self._43);
		cof_mat4._24 = temp.det();
		
		temp.set(self._12,self._13,self._14,self._22,self._23,self._24,self._42,self._43,self._44);
		cof_mat4._31 = temp.det();
		temp.set(self._11,self._13,self._14,self._21,self._23,self._24,self._41,self._43,self._44);
		cof_mat4._32 = -temp.det();
		temp.set(self._11,self._12,self._14,self._21,self._22,self._24,self._41,self._42,self._44);
		cof_mat4._33 = temp.det();
		temp.set(self._11,self._12,self._13,self._21,self._22,self._23,self._41,self._42,self._43);
		cof_mat4._34 = -temp.det();
		
		temp.set(self._12,self._13,self._14,self._22,self._23,self._24,self._32,self._33,self._34);
		cof_mat4._41 = -temp.det();
		temp.set(self._11,self._13,self._14,self._21,self._23,self._24,self._31,self._33,self._34);
		cof_mat4._42 = temp.det();
		temp.set(self._11,self._12,self._14,self._21,self._22,self._24,self._31,self._32,self._34);
		cof_mat4._43 = -temp.det();
		temp.set(self._11,self._12,self._13,self._21,self._22,self._23,self._31,self._32,self._33);
		cof_mat4._44 = temp.det();
		
		cof_mat4.transponse();
		cof_mat4 *= 1. / self.det();
		self.set_from(&mut cof_mat4);
	}

	pub fn invert_to(&self, m2: &mut Mat4) {
		let mut temp: Mat3 = Mat3::new_empty();
		
		temp.set(self._22,self._23,self._24,self._32,self._33,self._34,self._42,self._43,self._44);
		m2._11 = temp.det();
		temp.set(self._21,self._23,self._24,self._31,self._33,self._34,self._41,self._43,self._44);
		m2._12 = -temp.det();
		temp.set(self._21,self._22,self._24,self._31,self._32,self._34,self._41,self._42,self._44);
		m2._13 = temp.det();
		temp.set(self._21,self._22,self._23,self._31,self._32,self._33,self._41,self._42,self._43);
		m2._14 = -temp.det();
		
		temp.set(self._12,self._13,self._14,self._32,self._33,self._34,self._42,self._43,self._44);
		m2._21 = -temp.det();
		temp.set(self._11,self._13,self._14,self._31,self._33,self._34,self._41,self._43,self._44);
		m2._22 = temp.det();
		temp.set(self._11,self._12,self._14,self._31,self._32,self._34,self._41,self._42,self._44);
		m2._23 = -temp.det();
		temp.set(self._11,self._12,self._13,self._31,self._32,self._33,self._41,self._42,self._43);
		m2._24 = temp.det();
		
		temp.set(self._12,self._13,self._14,self._22,self._23,self._24,self._42,self._43,self._44);
		m2._31 = temp.det();
		temp.set(self._11,self._13,self._14,self._21,self._23,self._24,self._41,self._43,self._44);
		m2._32 = -temp.det();
		temp.set(self._11,self._12,self._14,self._21,self._22,self._24,self._41,self._42,self._44);
		m2._33 = temp.det();
		temp.set(self._11,self._12,self._13,self._21,self._22,self._23,self._41,self._42,self._43);
		m2._34 = -temp.det();
		
		temp.set(self._12,self._13,self._14,self._22,self._23,self._24,self._32,self._33,self._34);
		m2._41 = -temp.det();
		temp.set(self._11,self._13,self._14,self._21,self._23,self._24,self._31,self._33,self._34);
		m2._42 = temp.det();
		temp.set(self._11,self._12,self._14,self._21,self._22,self._24,self._31,self._32,self._34);
		m2._43 = -temp.det();
		temp.set(self._11,self._12,self._13,self._21,self._22,self._23,self._31,self._32,self._33);
		m2._44 = temp.det();
		
		m2.transponse();
		*m2 *= 1. / self.det();
	}
}