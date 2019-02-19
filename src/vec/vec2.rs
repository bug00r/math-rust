use std::ops::*;

#[derive(Clone)]
pub struct Vec2 {
	pub x: f32,
	pub y: f32,
}

impl Add<f32> for Vec2 {
	type Output = Vec2;

	fn add(self, rhs: f32) -> Vec2 {
		Vec2 { x: self.x + rhs, y: self.y + rhs }
    }
}

impl Add<Vec2> for Vec2 {
	type Output = Vec2;
	
	fn add(self, rhs: Vec2) -> Vec2 {
		Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<'a> Add<&'a Vec2> for &'a Vec2 {
	type Output = Vec2;
	
	fn add(self, rhs: &'a Vec2) -> Vec2 {
		Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign<f32> for Vec2 {
	fn add_assign(&mut self, rhs: f32){
		self.x += rhs;
		self.y += rhs;
    }
}

impl AddAssign<Vec2> for Vec2 {
	
	fn add_assign(&mut self, rhs: Vec2){
		self.x += rhs.x;
		self.y += rhs.y;
    }
}

impl<'a> AddAssign<&'a Vec2> for Vec2 {
	
	fn add_assign(&mut self, rhs: &'a Vec2){
		self.x += rhs.x;
		self.y += rhs.y;
    }
}

impl Sub<f32> for Vec2 {
	type Output = Vec2;

	fn sub(self, rhs: f32) -> Vec2 {
		Vec2 { x: self.x - rhs, y: self.y - rhs }
    }
}

impl Sub<Vec2> for Vec2 {
	type Output = Vec2;
	
	fn sub(self, rhs: Vec2) -> Vec2 {
		Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<'a> Sub<&'a Vec2> for &'a Vec2 {
	type Output = Vec2;
	
	fn sub(self, rhs: &'a Vec2) -> Vec2 {
		Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl SubAssign<f32> for Vec2 {
	fn sub_assign(&mut self, rhs: f32){
		self.x -= rhs;
		self.y -= rhs;
    }
}

impl SubAssign<Vec2> for Vec2 {
	
	fn sub_assign(&mut self, rhs: Vec2){
		self.x -= rhs.x;
		self.y -= rhs.y;
    }
}

impl<'a> SubAssign<&'a Vec2> for Vec2 {
	
	fn sub_assign(&mut self, rhs: &'a Vec2){
		self.x -= rhs.x;
		self.y -= rhs.y;
    }
}

impl PartialEq for Vec2 {
	fn eq(&self, other: &Vec2) -> bool {
		(self as *const _ == other as *const _) || (self.x == other.x && self.y == other.y)
	}
	fn ne(&self, other: &Vec2) -> bool {
		(self as *const _ != other as *const _) && (self.x != other.x || self.y != other.y)
	}
}
impl Eq for Vec2 {}

impl Mul<f32> for Vec2 {
	type Output = Vec2;

	fn mul(self, rhs: f32) -> Vec2 {
		Vec2 { x: self.x * rhs, y: self.y * rhs }
    }
}

impl Mul<Vec2> for Vec2 {
	type Output = f32;
	
	fn mul(self, rhs: Vec2) -> f32 {
		(self.x * rhs.x) + (self.y * rhs.y)
    }
}

impl<'a> Mul<&'a Vec2> for &'a Vec2 {
	type Output = f32;
	
	fn mul(self, rhs: &'a Vec2) -> f32 {
		(self.x * rhs.x) + (self.y * rhs.y)
    }
}

impl MulAssign<f32> for Vec2 {
	fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
		self.y *= rhs;
    }
}

impl ToString for Vec2 {
	fn to_string(&self) -> String {
		format!("x:{} y:{}", self.x, self.y)
	}
}

impl Vec2 {
	pub fn new_empty() -> Vec2 {
		Vec2{ x: 0., y: 0. }
	}

	pub fn new(x: f32, y :f32) -> Vec2 {
		Vec2 { x, y }
	}

	pub fn set(&mut self, x: f32, y: f32) {
		self.x = x;
		self.y = y;
	}
	
	pub fn set_from(&mut self, v2: &Vec2) {
		self.x = v2.x;
		self.y = v2.y;
	}
	
	pub fn scale(&mut self, x: f32, y: f32) {
		self.x *= x;
		self.y *= y;
	}
	
	pub fn copy_to(&self, v2: &mut Vec2) {
		v2.x = self.x;
		v2.y = self.y;
	}
	
	pub fn cross( &self, v2: &Vec2) -> f32{
		(self.x * v2.y) - (v2.x * self.y)
	}

	pub fn is_orthogonal(&self, v2: &Vec2) -> bool {
		((self * v2) == 0.)
	}
	
	pub fn len(&self) -> f32 {
		((self.x*self.x) + (self.y*self.y)).sqrt()
	}
	
	pub fn negate(&mut self) {
		*self *= -1.;
	}
	
	pub fn negate_to(&self, v2: &mut Vec2) {
		v2.x = -self.x;
		v2.y = -self.y;
	}
	
	pub fn normalize(&mut self) {
		*self *= 1. / self.len();
	}
	
	pub fn normalize_to(&self, v2: &mut Vec2) {
		v2.set_from(self);
		v2.normalize();
	}
	
	pub fn angle(&self, v2: &Vec2) -> f32 {
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