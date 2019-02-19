use std::f32::consts::PI;
use std::f32::{MIN, MAX};
use rand::{thread_rng, Rng};
use statistics::average::*;
use utils::*;

pub struct Noise {
	pub map: Vec<f32>,
	pub min: f32,
	pub max: f32,
	pub width: u32,
	pub height: u32,
}

impl Noise {
	pub fn new(width: u32, height: u32) -> Noise {
		let mut noise: Noise = Noise {
			min: MAX,
			max: MIN,
			width: width,
			height: height,
			map: Vec::with_capacity((width*height) as usize),
		};
		
		for _i in 0..noise.map.capacity() {
			noise.map.push(0.);
		}
		
		noise
	}

	pub fn create_noise_value(&mut self, min: f32, max: f32) -> f32 {
	  let mut rng = thread_rng();
	  let noiseval: f32 = rng.gen_range(min, max);
	  self.min = noiseval.min(self.min);
	  self.max = noiseval.max(self.max);
	  noiseval
	}

	pub fn setnewcolor(&mut self, seed: f32, target: usize) {
		self.map[target] = self.create_noise_value(-seed, seed);
	}
	
}

pub fn seed_reduction_add(seed: f32, reduction: f32) -> f32 {
	 seed + reduction
}
pub fn seed_reduction_sub(seed: f32, reduction: f32) -> f32 {
	 seed - reduction
}
pub fn seed_reduction_mul(seed: f32, reduction: f32) -> f32 {
	 seed * reduction
}
pub fn seed_reduction_div(seed: f32, reduction: f32) -> f32 {
	 seed / reduction
}
pub fn seed_reduction_add_sqrt(seed: f32, reduction: f32) -> f32 {
	 seed + reduction.sqrt()
}
pub fn seed_reduction_sub_sqrt(seed: f32, reduction: f32) -> f32 {
	 seed - reduction.sqrt()
}
pub fn seed_reduction_mul_sqrt(seed: f32, reduction: f32) -> f32 {
	 seed * reduction.sqrt()
}
pub fn seed_reduction_div_sqrt(seed: f32, reduction: f32) -> f32 {
	 seed / reduction.sqrt()
}
pub fn seed_reduction_pow2_p_r_div_pow2_m_r(seed: f32, reduction: f32) -> f32 {
	 seed * (((seed*seed) + reduction)/((seed*seed) - reduction))
}
pub fn seed_reduction_pow2_m_r_div_pow2_p_r(seed: f32, reduction: f32) -> f32 {
	 seed * (((seed*seed) - reduction)/((seed*seed) + reduction))
}
pub fn seed_reduction_arithmetic_add(seed: f32, reduction: f32) -> f32 
{
	 seed + middle_arithmetic2(seed, reduction)
}
pub fn seed_reduction_arithmetic_sub(seed: f32, reduction: f32) -> f32 
{
	 seed - middle_arithmetic2(seed, reduction)
}
pub fn seed_reduction_arithmetic_mul(seed: f32, reduction: f32) -> f32 
{
	 seed * middle_arithmetic2(seed, reduction)
}
pub fn seed_reduction_arithmetic_div(seed: f32, reduction: f32) -> f32 
{
	 seed / middle_arithmetic2(seed, reduction)
}
pub fn seed_reduction_hoelder_add(seed: f32, reduction: f32) -> f32 
{
	 seed + middle_hoelder2(seed, reduction)
}
pub fn seed_reduction_hoelder_sub(seed: f32, reduction: f32) -> f32 
{
	 seed - middle_hoelder2(seed, reduction)
}
pub fn seed_reduction_hoelder_mul(seed: f32, reduction: f32) -> f32 
{
	 seed * middle_hoelder2(seed, reduction)
}
pub fn seed_reduction_hoelder_div(seed: f32, reduction: f32) -> f32 
{
	 seed / middle_hoelder2(seed, reduction)
}
pub fn seed_reduction_geometric_add(seed: f32, reduction: f32) -> f32 
{
	 seed + middle_geometric2(seed, reduction)
}
pub fn seed_reduction_geometric_sub(seed: f32, reduction: f32) -> f32 
{
	 seed - middle_geometric2(seed, reduction)
}
pub fn seed_reduction_geometric_mul(seed: f32, reduction: f32) -> f32 
{
	 seed * middle_geometric2(seed, reduction)
}
pub fn seed_reduction_geometric_div(seed: f32, reduction: f32) -> f32 
{
	 seed / middle_geometric2(seed, reduction)
}
pub fn seed_reduction_cubic_add(seed: f32, reduction: f32) -> f32 
{
	 seed + middle_cubic2(seed, reduction)
}
pub fn seed_reduction_cubic_sub(seed: f32, reduction: f32) -> f32 
{
	 seed - middle_cubic2(seed, reduction)
}
pub fn seed_reduction_cubic_mul(seed: f32, reduction: f32) -> f32 
{
	 seed * middle_cubic2(seed, reduction)
}
pub fn seed_reduction_cubic_div(seed: f32, reduction: f32) -> f32 
{
	 seed / middle_cubic2(seed, reduction)
}
pub fn seed_reduction_quad_add(seed: f32, reduction: f32) -> f32 
{
	 seed + middle_quad2(seed, reduction)
}
pub fn seed_reduction_quad_sub(seed: f32, reduction: f32) -> f32 
{
	 seed - middle_quad2(seed, reduction)
}
pub fn seed_reduction_quad_mul(seed: f32, reduction: f32) -> f32 
{
	 seed * middle_quad2(seed, reduction)
}
pub fn seed_reduction_quad_div(seed: f32, reduction: f32) -> f32 
{
	 seed / middle_quad2(seed, reduction)
}
pub fn seed_reduction_harmonic_add(seed: f32, reduction: f32) -> f32 
{
	 seed + middle_harmonic2(seed, reduction)
}
pub fn seed_reduction_harmonic_sub(seed: f32, reduction: f32) -> f32 
{
	 seed - middle_harmonic2(seed, reduction)
}
pub fn seed_reduction_harmonic_mul(seed: f32, reduction: f32) -> f32 
{
	 seed * middle_harmonic2(seed, reduction)
}
pub fn seed_reduction_harmonic_div(seed: f32, reduction: f32) -> f32 
{
	 seed / middle_harmonic2(seed, reduction)
}

pub fn filter_noise_gauss( noise: &mut Noise, deviation: f32) {
	let nw: i32 = noise.width as i32;
	let nh: i32 = noise.height as i32;
	
	let kernelside: i32 = nw;
	let mut gausskernel: Vec<f32> = Vec::with_capacity((kernelside*kernelside) as usize);
	
	let useddeviavtion:f32 = 2.*deviation*deviation;
	let base:f32=1./(PI*useddeviavtion);
	
	let pxrange = nw >> 1;
	
	let mut ky = 0;
	for pry in -pxrange..pxrange {
		let y_2: f32 = (pry*pry) as f32;
		let cur_h: usize = (ky * kernelside) as usize;
		let mut kx: usize = 0;
		
		for prx in -pxrange..pxrange {
			let x_2: f32 = (prx*prx) as f32;
			gausskernel[cur_h + kx] = base * -((y_2+x_2)/useddeviavtion).exp();
			kx += 1;
		}
		ky += 1;
	}
	
	let mut noise_min: f32 = MAX;
	let mut noise_max: f32 = MIN;

	for h in 0..nh {
		let cur_h: usize = (h * nw) as usize;
		for w in 0..nw {
			let idx: usize = cur_h + w as usize;
			let fref = noise.map[idx];

			let factor: f32 = gausskernel[(h * kernelside + w) as usize];
			let mut noiseval = interpolate_lin(fref, noise.min, 0., noise.max, 1.)*factor;

			noise_min = noise_min.min(noiseval);
			noise_max = noise_max.max(noiseval);
			noise.map[idx] = noiseval;
		}
	}

	noise.min = noise_min;
	noise.max = noise_max;
	
}		

pub fn filter_noise_circle( noise: &mut Noise, radius: f32) {
	let nw: i32 = noise.width as i32;
	let nh: i32 = noise.height as i32;
	
	let pxrange: f32 = (nw >> 1) as f32;
	
	let max_len: f32 = (2.*pxrange*pxrange).sqrt();
	let inner_limit: f32 = max_len * radius;
	
	let mut noise_min: f32 = MAX;
	let mut noise_max: f32 = MIN;
	
	for h in 0..nh {
		let cur_h: usize = (h * nw) as usize;
		for w in 0..nw {
			let idx: usize = cur_h + w as usize;
			let fref = noise.map[idx];

			let mut noiseval: f32 = 0.;
			
			let x: f32 = pxrange - w as f32;
			let y: f32 = pxrange - h as f32;
			let cur_len: f32 = ((x*x) + (y*y)).sqrt();
			
			if cur_len <= inner_limit {
					let factor: f32 = 1.-(cur_len/inner_limit);
					noiseval = interpolate_lin(fref, noise.min, 0., noise.max, 1.)*factor;
			}
			
			noise_min = noise_min.min(noiseval);
			noise_max = noise_max.max(noiseval);
			noise.map[idx] = noiseval;
	    }
	}
	
	noise.min = noise_min;
	noise.max = noise_max;
}
