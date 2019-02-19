use std::cmp::Ordering::Equal;

pub fn middle_arithmetic(x1: f32, x2: f32, x3: f32, x4: f32) ->f32 {
	(x1 + x2 + x3 + x4)*0.25
}

pub fn middle_arithmetic2(x1: f32, x2: f32) ->f32 {
	(x1 + x2)*0.5
}

 
pub fn middle_harmonic(x1: f32, x2: f32, x3: f32, x4: f32) -> f32{
	4./(1./x1 + 1./x2 + 1./x3 + 1./x4)
}

 
pub fn middle_harmonic2(x1: f32, x2: f32) -> f32{
	2./(1./x1 + 1./x2)
}

 
pub fn middle_quantil(x1: f32, x2: f32, x3: f32, x4: f32) -> f32{
	let mut values: [f32; 4] = [ x1, x2, x3, x4 ];
	
	values.sort_by(| a, b | a.partial_cmp(b).unwrap_or(Equal));
	
	(values[0] + values[2])/2.
}

pub fn middle_median(x1: f32, x2: f32, x3: f32, x4: f32) -> f32{
	let mut values: [f32; 4] = [ x1, x2, x3, x4 ];
	
	values.sort_by(| a, b | a.partial_cmp(b).unwrap_or(Equal));
	
	(values[1] + values[2])/2.
}
 
pub fn middle_quad(x1: f32, x2: f32, x3: f32, x4: f32) -> f32 {
	(((x1*x1)+(x2*x2)+(x3*x3)+(x4*x4))*0.25).sqrt()
}

pub fn middle_quad2(x1: f32, x2: f32) -> f32 {
	(((x1*x1)+(x2*x2))*0.5).sqrt()
}

pub fn middle_cubic(x1: f32, x2: f32, x3: f32, x4: f32) -> f32 {
	(((x1*x1*x1)+(x2*x2*x2)+(x3*x3*x3)+(x4*x4*x4))*0.25).cbrt()
}
 
pub fn middle_cubic2(x1: f32, x2: f32) -> f32 {
	(((x1*x1*x1)+(x2*x2*x2))*0.5).cbrt()
}
 
pub fn middle_geometric(x1: f32, x2: f32, x3: f32, x4: f32) -> f32 {
	(x1*x2*x3*x4).powf(0.25)
}
 
pub fn middle_geometric2(x1: f32, x2: f32) -> f32 {
	(x1*x2).powf(0.5)
}
 
pub fn middle_hoelder(x1: f32, x2: f32, x3: f32, x4: f32) -> f32 {
	(((x1*x1*x1*x1)+(x2*x2*x2*x2)+(x3*x3*x3*x3)+(x4*x4*x4*x4))*0.25).powf(0.25)
}
 
pub fn middle_hoelder2(x1: f32, x2: f32) -> f32 {
	(((x1*x1)+(x2*x2))*0.5).powf(0.5)
}
