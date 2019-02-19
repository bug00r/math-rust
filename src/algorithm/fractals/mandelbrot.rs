use num_complex::*;
extern crate rand;

#[derive(Clone)]
pub struct MandelbrotPoint {
	pub cpoint: Complex32,
	pub spoint: Complex32,
	pub abs: f32,
	pub iterations: i32,
	pub isin: bool,
}

impl MandelbrotPoint {
	pub fn new() -> MandelbrotPoint {
		MandelbrotPoint{ cpoint: Complex { re: 0.0, im: 0.0 }, spoint: Complex { re: 0.0, im: 0.0 }, abs: 0., iterations: 0, isin: false }
	}
}

pub fn inside_mandelbrot_set ( point: Complex32, cntiterations: i32, result: &mut MandelbrotPoint){
	let mut cur_omplex: Complex32 = Complex{ re: 0., im: 0.};
	let mut curabs: f32 = 0.;
	let mut insinde: bool = true;
	let mut needed_i: i32 = 0;
	for i in 1..=cntiterations {
		needed_i = i;
		cur_omplex = cur_omplex.powf(2.) + point;
		curabs = ((cur_omplex.re * cur_omplex.re) + (cur_omplex.im * cur_omplex.im)).sqrt();
		if curabs > 2.{
			insinde = false;
			break;
		}
	}
	result.cpoint = cur_omplex;
	result.spoint = point;
	result.abs = curabs;
	result.iterations = needed_i;
	result.isin = insinde;
}

pub struct Mandelbrot {
	pub min: Complex32,
	pub max: Complex32,
	pub width: u32,
	pub height: u32,
	pub cntiterations: i32,
	pub map : Vec<MandelbrotPoint>,
}

impl Mandelbrot {
	pub fn new( width: u32, height: u32) -> Mandelbrot {
		let mut mb = Mandelbrot {
			min: Complex { re: 0.0, im: 0.0 },
			max: Complex { re: 0.0, im: 0.0 },
			width: width,
			height: height,
			cntiterations: 20,
			map: Vec::with_capacity((width*height) as usize),
		};
		
		for _i in 0..mb.map.capacity() {
			mb.map.push(MandelbrotPoint::new());
		}
		mb
	}
	
	pub fn create(&mut self) {
		let mut cur: Complex32 = self.min.clone(); 
		let mwidth = self.width;
		let mheight = self.height;
		let step: Complex32 = Complex { re: (self.max.re - self.min.re) / mwidth as f32, im: (self.max.im - self.min.im) / mheight as f32 };
		
		for curh in 0..mheight {
			for curw in 0..mwidth {			
				inside_mandelbrot_set(cur.clone(), self.cntiterations, &mut self.map[(curh * mwidth + curw) as usize]);
				cur.re += step.re;
			}
			cur.re = self.min.re;
			cur.im += step.im;
		}
	}
	
}
