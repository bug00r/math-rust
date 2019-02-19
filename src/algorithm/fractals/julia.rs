use num_complex::*;
extern crate rand;

#[derive(Clone)]
pub struct JuliaPoint {
	pub iterations: i32,
	pub isin: bool,
	pub cpoint: Complex32,
	pub spoint: Complex32,
	pub abs: f32,
}

impl JuliaPoint {
	pub fn new() -> JuliaPoint {
		JuliaPoint{ cpoint: Complex { re: 0.0, im: 0.0 }, spoint: Complex { re: 0.0, im: 0.0 }, abs: 0., iterations: 0, isin: false }
	}
}

pub fn inside_julia_set (point: Complex32 , c: &Complex32, cntiterations: i32, result: &mut JuliaPoint,
				polyfunc: fn (&Complex32, &Complex32) -> Complex32)
{
	let mut cur_complex: Complex32 = point.clone();
	let mut curabs: f32 = 0.;
	let mut insinde: bool = true;
	let mut needed_i: i32 = 0;
	for i in 1..=cntiterations {
		needed_i = i;
		cur_complex = polyfunc(&cur_complex, c);
		curabs = ((cur_complex.re * cur_complex.re) + (cur_complex.im * cur_complex.im)).sqrt();
		if curabs > 2. {
			insinde = true;
			break;
		}
	}
	result.cpoint = cur_complex;
	result.spoint = point;
	result.abs = curabs;
	result.iterations = needed_i;
	result.isin = insinde;
}

pub struct Julia{
	pub min: Complex32,
	pub max: Complex32,
	pub width: u32,
	pub height: u32,
	pub cntiterations: i32,
	pub c: Complex32,
	pub polyfunc: fn(&Complex32, &Complex32) -> Complex32,
	pub map : Vec<JuliaPoint>
}

impl Julia {
	pub fn new( width: u32, height: u32) -> Julia {
		let mut julia = Julia {
			min: Complex { re: 0.0, im: 0.0 },
			max: Complex { re: 0.0, im: 0.0 },
			c: Complex { re: 0.0, im: 0.0 },
			width: width,
			height: height,
			cntiterations: 20,
			polyfunc: julia_pfunc_quad_plus_c,
			map: Vec::with_capacity((width*height) as usize),
		};
		
		for _i in 0..julia.map.capacity() {
			julia.map.push(JuliaPoint::new());
		}
		julia
	}
	
	pub fn create(&mut self) {
		let mut cur: Complex32 = self.min.clone(); 
		let mwidth = self.width;
		let mheight = self.height;
		let step: Complex32 = Complex { re: (self.max.re - self.min.re) / mwidth as f32, im: (self.max.im - self.min.im) / mheight as f32 };
		
		for curh in 0..mheight {
			for curw in 0..mwidth {		
				inside_julia_set(cur.clone(), &self.c, self.cntiterations, &mut self.map[(curh * mwidth + curw) as usize], self.polyfunc);
				cur.re += step.re;
			}
			cur.re = self.min.re;
			cur.im += step.im;
		}
	
	}
	
}

pub fn julia_pfunc_quad_plus_c(cp: &Complex32, c: &Complex32) -> Complex32{
	(cp * cp) + c
}

pub fn julia_pfunc_third_plus_c(cp: &Complex32, c: &Complex32) -> Complex32{
	(cp * cp * cp) + c
}

pub fn julia_pfunc_fourth_plus_c(cp: &Complex32, c: &Complex32) -> Complex32{
	(cp * cp * cp * cp) + c
}

pub fn julia_pfunc_exp_plus_c(cp: &Complex32, c: &Complex32) -> Complex32{
	(cp * cp * cp).exp() - c
}

pub fn julia_pfunc_ten_plus_c(cp: &Complex32, c: &Complex32) -> Complex32{
	(cp * cp * cp * cp * cp * cp * cp * cp * cp * cp) + c
}

pub fn julia_pfunc_quad_plus_c_2(cp: &Complex32, c: &Complex32) -> Complex32{
	let pow2_z = cp * cp;
	(pow2_z + c) / (pow2_z - c)
}

pub fn julia_pfunc_quad_plus_c_1_2(cp: &Complex32, c: &Complex32) -> Complex32{
	let pow2_z = cp * cp;
	(pow2_z - c) / (pow2_z + c)
}

pub fn julia_pfunc_sqrt_plus_c(cp: &Complex32, c: &Complex32) -> Complex32{
	cp.sqrt() + c
}

pub fn julia_pfunc_sqrt_minus_c(cp: &Complex32, c: &Complex32) -> Complex32{
	cp.sqrt() - c
}

pub fn julia_pfunc_quad_plus_c_3(cp: &Complex32, c: &Complex32) -> Complex32{
	let pow3_z = cp * cp * cp;
	(pow3_z + c) / (pow3_z - c)
}

pub fn julia_pfunc_3_2_div_pc(cp: &Complex32, c: &Complex32) -> Complex32{
	let pow2 = cp * cp;
	let pow3 = pow2 * cp;
	let calc = (cp - (pow2 / 2.)) * (cp - (pow2 / 2.));
	((1. - (pow3/6.)) / calc) + c
}

pub fn julia_pfunc_pow_3_p_2_p_1_p_c(cp: &Complex32, c: &Complex32) -> Complex32{
	let pow2 = cp * cp;
	let pow3 = pow2 * cp;
	pow3 + pow2 + cp + c
}

pub fn julia_pfunc_px_random(cp: &Complex32, c: &Complex32) -> Complex32{
	let fdix = rand::random::<f32>() as i32;
	match fdix {
		0 => return julia_pfunc_quad_plus_c_2(cp, c),
		1 => return julia_pfunc_quad_plus_c_1_2(cp, c),
		_ => return julia_pfunc_quad_plus_c_2(cp, c),
	}
}