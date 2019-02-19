use algorithm::noise::core::*;
use statistics::average::*;
use rand::{thread_rng, Rng};

pub struct MidpointDisplacement {
	pub noise: Noise,
	pub length: i32,
	pub startseed: f32, 
	pub seed: f32, 
	pub reduction: f32,
	pub middlefunc: fn(x1: f32, x2: f32, x3: f32, x4: f32) -> f32,
	pub middlefunc2: fn(x1: f32, x2: f32) -> f32,
	pub seedreducefunc: fn(seed: f32, reduction: f32) -> f32,
}

impl MidpointDisplacement {
	pub fn new(width: u32, height: u32) -> MidpointDisplacement {
		MidpointDisplacement {
			noise: Noise::new(width, height),
			length: 0,
			startseed: 0.,
			seed: 0.,
			reduction: 0.,
			middlefunc: middle_arithmetic,
			middlefunc2: middle_arithmetic2,
			seedreducefunc: seed_reduction_mul,
		}
	}
	
	pub fn create(&mut self) {
		let length: i32 = self.length;
		let startseed: f32 = self.startseed;
		let seed: f32 = self.seed;
		let reduction: f32 = self.reduction;
		let middlefunc = self.middlefunc;
		let middlefunc2 = self.middlefunc2;
		let seedreducefunc = self.seedreducefunc;
		
		let noise_width: i32 = self.noise.width as i32;
		let cntsquare: i32 = (noise_width - 1) / length;
		let middle: i32 = length >> 1;
		//this bracket is for only one mutable borrow of noise regarding recursion self.create()
		{
			let noise: &mut Noise = &mut self.noise;

			
			
			if cntsquare == 1 {
				noise.setnewcolor(startseed, 0);
				noise.setnewcolor(startseed, length as usize);
				noise.setnewcolor(startseed, (length*length+length) as usize);
				noise.setnewcolor(startseed, (length*length + (2*length)) as usize);
			}
			
			let mut rng = thread_rng();
			
			let mut colval: f32;
			
			for sqy in 0..cntsquare {
				for sqx in 0..cntsquare {
				
					let lt_val: f32 = noise.map[(sqy * length * noise_width + (sqx*length)) as usize];
					let rt_val: f32 = noise.map[(sqy * length * noise_width + ((sqx+1)*length)) as usize];
					let lb_val: f32 = noise.map[((sqy+1) * length * noise_width + (sqx*length)) as usize];
					let rb_val: f32 = noise.map[((sqy+1) * length * noise_width + ((sqx+1)*length)) as usize];
					
					let x: i32 = (sqx*length) + middle;
					let y: i32 = (sqy*length) + middle;
					
					colval = middlefunc2(lt_val, rt_val) + rng.gen_range(-seed, seed);
					noise.min = noise.min.min(colval);
					noise.max = noise.max.max(colval);
					noise.map[((y-middle) * noise_width + x) as usize] = colval;
					
					colval = middlefunc2(lb_val, rb_val) + rng.gen_range(-seed, seed);
					noise.min = noise.min.min(colval);
					noise.max = noise.max.max(colval);
					noise.map[((y+middle) * noise_width + x) as usize] = colval;
					
					colval = middlefunc2(lt_val, lb_val) + rng.gen_range(-seed, seed);
					noise.min = noise.min.min(colval);
					noise.max = noise.max.max(colval);
					noise.map[(y * noise_width + x - middle) as usize] = colval;
					
					colval = middlefunc2(rt_val, rb_val) + rng.gen_range(-seed, seed);
					noise.min = noise.min.min(colval);
					noise.max = noise.max.max(colval);
					noise.map[(y * noise_width + x + middle) as usize] = colval;
					
					colval = middlefunc( noise.map[((y-middle) * noise_width + x) as usize],
										 noise.map[((y+middle) * noise_width + x) as usize],
										 noise.map[(y * noise_width + x - middle) as usize],
										 noise.map[(y * noise_width + x + middle) as usize]);
											
					colval += rng.gen_range(-seed, seed);
					noise.min = noise.min.min(colval);
					noise.max = noise.max.max(colval);
					noise.map[(y * noise_width + x) as usize] = colval;	
				
				}
			}
			
		}
		if middle >= 2 {
			self.seed = seedreducefunc(seed, reduction);
			self.length = middle;
			self.create();
		}
	}	
}
