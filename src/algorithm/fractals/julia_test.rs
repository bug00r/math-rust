use algorithm::fractals::julia::*;
use num_complex::*;

#[test]
fn julia_set() {	
	let mut julia: Julia = Julia::new(512, 512);
	julia.min.re = 0.;
	julia.max.re = 0.5;
	julia.min.im = 0.;
	julia.max.im = 0.3;
	julia.cntiterations = 200;
	julia.c.re = 0.;
	julia.c.im = 1.;
	
	julia.create();
}

#[test]
fn julia_set_polyfunc_random() {	
	let mut julia: Julia = Julia::new(512, 512);
	julia.min.re = 0.;
	julia.max.re = 0.5;
	julia.min.im = 0.;
	julia.max.im = 0.3;
	julia.cntiterations = 200;
	julia.c.re = 0.;
	julia.c.im = 1.;
	
	julia.polyfunc = julia_pfunc_px_random;
	
	julia.create();
}

