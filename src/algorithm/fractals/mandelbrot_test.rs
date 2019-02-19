use algorithm::fractals::mandelbrot::*;
use num_complex::*;

#[test]
fn mandelbrot_inside() {	
	let its: i32 = 20;
	let cp: Complex32 = Complex{ re: -2., im: 1.};
	let cp2: Complex32 = Complex{ re: 0.3, im: 0.3};
	let mut mb: MandelbrotPoint = MandelbrotPoint::new();
	
	inside_mandelbrot_set(cp, its, &mut mb);
	
	assert_eq!(mb.isin, false);
		
	inside_mandelbrot_set(cp2, its, &mut mb);
	assert_eq!(mb.isin, true);
}

#[test]
fn mandelbrot_set() {	
	let mut mb: Mandelbrot = Mandelbrot::new(512, 512);
	mb.min.re = -2.;
	mb.max.re = 0.5;
	mb.min.im = -1.;
	mb.max.im = 1.;
	mb.cntiterations = 20;
	mb.create();
}
