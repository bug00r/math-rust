use std::ops::*;
use mat::mat3::*;

#[derive(Clone)]
pub struct Vec3 {
	pub x: f32,
	pub y: f32,
	pub z: f32,
}

impl Add<f32> for Vec3 {
	type Output = Vec3;

	fn add(self, rhs: f32) -> Vec3 {
		Vec3 { x: self.x + rhs, y: self.y + rhs, z: self.z + rhs }
    }
}

impl Add<Vec3> for Vec3 {
	type Output = Vec3;
	
	fn add(self, rhs: Vec3) -> Vec3 {
		Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl<'a> Add<&'a Vec3> for &'a Vec3 {
	type Output = Vec3;
	
	fn add(self, rhs: &'a Vec3) -> Vec3 {
		Vec3 { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl AddAssign<f32> for Vec3 {
	fn add_assign(&mut self, rhs: f32){
		self.x += rhs;
		self.y += rhs;
		self.z += rhs;
    }
}

impl AddAssign<Vec3> for Vec3 {
	
	fn add_assign(&mut self, rhs: Vec3){
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
    }
}

impl<'a> AddAssign<&'a Vec3> for Vec3 {
	
	fn add_assign(&mut self, rhs: &'a Vec3){
		self.x += rhs.x;
		self.y += rhs.y;
		self.z += rhs.z;
    }
}

impl Sub<f32> for Vec3 {
	type Output = Vec3;

	fn sub(self, rhs: f32) -> Vec3 {
		Vec3 { x: self.x - rhs, y: self.y - rhs, z: self.z - rhs }
    }
}

impl Sub<Vec3> for Vec3 {
	type Output = Vec3;
	
	fn sub(self, rhs: Vec3) -> Vec3 {
		Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl<'a> Sub<&'a Vec3> for &'a Vec3 {
	type Output = Vec3;
	
	fn sub(self, rhs: &'a Vec3) -> Vec3 {
		Vec3 { x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z }
    }
}

impl SubAssign<f32> for Vec3 {
	fn sub_assign(&mut self, rhs: f32){
		self.x -= rhs;
		self.y -= rhs;
		self.z -= rhs;
    }
}

impl SubAssign<Vec3> for Vec3 {
	
	fn sub_assign(&mut self, rhs: Vec3){
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
    }
}

impl<'a> SubAssign<&'a Vec3> for Vec3 {
	
	fn sub_assign(&mut self, rhs: &'a Vec3){
		self.x -= rhs.x;
		self.y -= rhs.y;
		self.z -= rhs.z;
    }
}

impl PartialEq for Vec3 {
	fn eq(&self, other: &Vec3) -> bool {
		(self as *const _ == other as *const _) || (self.x == other.x && self.y == other.y && self.z == other.z)
	}
	fn ne(&self, other: &Vec3) -> bool {
		(self as *const _ != other as *const _) && (self.x != other.x || self.y != other.y || self.z != other.z)
	}
}
impl Eq for Vec3 {}

impl Mul<f32> for Vec3 {
	type Output = Vec3;

	fn mul(self, rhs: f32) -> Vec3 {
		Vec3 { x: self.x * rhs, y: self.y * rhs, z: self.z * rhs }
    }
}

impl Mul<Vec3> for Vec3 {
	type Output = f32;
	
	fn mul(self, rhs: Vec3) -> f32 {
		(self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

impl<'a> Mul<&'a Vec3> for &'a Vec3 {
	type Output = f32;
	
	fn mul(self, rhs: &'a Vec3) -> f32 {
		(self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }
}

impl Mul<Mat3> for Vec3 {
	type Output = Vec3;
	
	fn mul(self, rhs: Mat3) -> Vec3 {
		Vec3 {
			x: (rhs._11 * self.x) + (rhs._12 * self.y) + (rhs._13 * self.z),
			y: (rhs._21 * self.x) + (rhs._22 * self.y) + (rhs._23 * self.z),
			z: (rhs._31 * self.x) + (rhs._32 * self.y) + (rhs._33 * self.z)
		}
    }
}

impl<'a> Mul<&'a Mat3> for &'a Vec3 {
	type Output = Vec3;
	
	fn mul(self, rhs: &'a Mat3) -> Vec3 {
		Vec3 {
			x: (rhs._11 * self.x) + (rhs._12 * self.y) + (rhs._13 * self.z),
			y: (rhs._21 * self.x) + (rhs._22 * self.y) + (rhs._23 * self.z),
			z: (rhs._31 * self.x) + (rhs._32 * self.y) + (rhs._33 * self.z)
		}
    }
}

impl MulAssign<f32> for Vec3 {
	fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
		self.y *= rhs;
		self.z *= rhs;
    }
}

impl MulAssign<Mat3> for Vec3 {
	fn mul_assign(&mut self, rhs: Mat3) {
		let temp: Vec3 = self.clone();
        self.x = (rhs._11 * temp.x) + (rhs._12 * temp.y) + (rhs._13 * temp.z);
		self.y = (rhs._21 * temp.x) + (rhs._22 * temp.y) + (rhs._23 * temp.z);
		self.z = (rhs._31 * temp.x) + (rhs._32 * temp.y) + (rhs._33 * temp.z);
    }
}

impl<'a> MulAssign<&'a Mat3> for Vec3 {
	fn mul_assign(&mut self, rhs: &'a Mat3) {
		let temp: Vec3 = self.clone();
        self.x = (rhs._11 * temp.x) + (rhs._12 * temp.y) + (rhs._13 * temp.z);
		self.y = (rhs._21 * temp.x) + (rhs._22 * temp.y) + (rhs._23 * temp.z);
		self.z = (rhs._31 * temp.x) + (rhs._32 * temp.y) + (rhs._33 * temp.z);
    }
}

impl ToString for Vec3 {
	fn to_string(&self) -> String {
		format!("x:{} y:{} z:{}", self.x, self.y, self.z)
	}
}

impl Vec3 {
	pub fn new_empty() -> Vec3 {
		Vec3{ x: 0., y: 0., z: 0.}
	}

	pub fn new(x: f32, y :f32, z :f32) -> Vec3 {
		Vec3 { x, y, z}
	}

	pub fn set(&mut self, x: f32, y: f32, z :f32) {
		self.x = x;
		self.y = y;
		self.z = z;
	}
	
	pub fn set_from(&mut self, v2: &Vec3) {
		self.x = v2.x;
		self.y = v2.y;
		self.z = v2.z;
	}
	
	pub fn scale(&mut self, x: f32, y: f32, z :f32) {
		self.x *= x;
		self.y *= y;
		self.z *= z;
	}
	
	pub fn copy_to(&self, v2: &mut Vec3) {
		v2.x = self.x;
		v2.y = self.y;
		v2.z = self.z;
	}

	pub fn cross( &self, v2: &Vec3) -> Vec3{		
		Vec3 {
			x: (self.y * v2.z) - (self.z * v2.y),
			y: (self.z * v2.x) - (self.x * v2.z),
			z: (self.x * v2.y) - (self.y * v2.x)
		}
	}
	
	pub fn cross_to( &self, v2: &Vec3, target: &mut Vec3){
		target.x = (self.y * v2.z) - (self.z * v2.y);
		target.y = (self.z * v2.x) - (self.x * v2.z);
		target.z = (self.x * v2.y) - (self.y * v2.x);
	}

	pub fn spat(&self, v2: &Vec3, v3: &Vec3) -> f32 {
		let mut cross: Vec3 = Vec3::new_empty();
		self.cross_to(v2, &mut cross);
		&cross * v3
	}
	
	pub fn is_orthogonal(&self, v2: &Vec3) -> bool {
		((self * v2) == 0.)
	}
	
	pub fn len(&self) -> f32 {
		((self.x*self.x) + (self.y*self.y) + (self.z*self.z)).sqrt()
	}

	
	pub fn mul_mat3_to(&self, mat: &Mat3, target: &mut Vec3) {
		target.x = (mat._11 * self.x) + (mat._12 * self.y) + (mat._13 * self.z);
		target.y = (mat._21 * self.x) + (mat._22 * self.y) + (mat._23 * self.z);
		target.z = (mat._31 * self.x) + (mat._32 * self.y) + (mat._33 * self.z);
	}
	
	pub fn negate(&mut self) {
		*self *= -1.;
	}
	
	pub fn negate_to(&self, v2: &mut Vec3) {
		v2.x = -self.x;
		v2.y = -self.y;
		v2.z = -self.z;
	}
	
	pub fn normalize(&mut self) {
		*self *= 1. / self.len();
	}
	
	pub fn normalize_to(&self, v2: &mut Vec3) {
		v2.set_from(self);
		v2.normalize();
	}
	
	pub fn angle(&self, v2: &Vec3) -> f32 {
		((self * v2) / (self.len() * v2.len())).acos()
	}
		
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests_private {
    use super::*;

    //#[test]
    //fn two_is_two() {
    //    assert_eq!(two(), 2);
    //}
}