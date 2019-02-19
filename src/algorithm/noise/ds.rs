use algorithm::noise::core::*;
use statistics::average::*;
use rand::{thread_rng, Rng};

pub struct DiamondSquare {
	pub noise: Noise,
	pub length: i32,
	pub startseed: f32, 
	pub seed: f32, 
	pub reduction: f32,
	pub middlefunc: fn(x1: f32, x2: f32, x3: f32, x4: f32) -> f32,
	pub seedreducefunc: fn(seed: f32, reduction: f32) -> f32,
}

impl DiamondSquare {
	pub fn new(width: u32, height: u32) -> DiamondSquare {
		DiamondSquare {
			noise: Noise::new(width, height),
			length: 0,
			startseed: 0.,
			seed: 0.,
			reduction: 0.,
			middlefunc: middle_arithmetic,
			seedreducefunc: seed_reduction_mul,
		}
	}
	
	pub fn create(&mut self) {
		let length: i32 = self.length;
		let startseed: f32 = self.startseed;
		let seed: f32 = self.seed;
		let reduction: f32 = self.reduction;
		let middlefunc = self.middlefunc;
		let seedreducefunc = self.seedreducefunc;
		
		let noise_width: i32 = self.noise.width as i32;
		let usedwidth: i32 = (noise_width - 1) as i32;
		let cntsquare: i32 = (noise_width - 1) / length;
		let middle: i32 = length >> 1;
		//this bracket is for only one mutable borrow of noise regarding recursion self.create()
		{
			let noise: &mut Noise = &mut self.noise;

			let mut colval: f32;
			
			if cntsquare == 1 {
				noise.setnewcolor(startseed, 0);
				noise.setnewcolor(startseed, length as usize);
				noise.setnewcolor(startseed, (length*length+length) as usize);
				noise.setnewcolor(startseed, (length*length + (2*length)) as usize);
			}
			
			let mut rng = thread_rng();
			
			for sqy in 0..cntsquare {
				for sqx in 0..cntsquare {
					////for each square
					let lt: usize = (sqy * length * noise_width + (sqx*length)) as usize;
					let rt: usize = (sqy * length * noise_width + ((sqx+1)*length)) as usize;
					let lb: usize = ((sqy+1) * length * noise_width + (sqx*length)) as usize;
					let rb: usize = ((sqy+1) * length * noise_width + ((sqx+1)*length)) as usize;
					
					////middlepoint
					let x: i32 = (sqx*length) + middle;
					let y: i32 = (sqy*length) + middle;

					colval = middlefunc(noise.map[lt], noise.map[rt], noise.map[lb], noise.map[rb]);
					colval += rng.gen_range(-seed, seed);		
					noise.min = noise.min.min(colval);
					noise.max = noise.max.max(colval);
					noise.map[(y * noise_width + x) as usize] = colval;
				}
			}
			
			for sqy in 0..cntsquare {
				for sqx in 0..cntsquare {
					
					////for each square			
					let lt_val: f32 = noise.map[(sqy * length * noise_width + (sqx*length)) as usize];
					let rt_val: f32 = noise.map[(sqy * length * noise_width + ((sqx+1)*length)) as usize];
					let lb_val: f32 = noise.map[((sqy+1) * length * noise_width + (sqx*length)) as usize];
					let rb_val: f32 = noise.map[((sqy+1) * length * noise_width + ((sqx+1)*length)) as usize];

					////middlepoint
					let x: i32 = (sqx*length) + middle;
					let y: i32 = (sqy*length) + middle;
					//
					let middleidx_val: f32 = noise.map[(y * noise_width + x)  as usize];
					////sidepoints
					////up
					let mut y2: i32 = y - length;
					if y2 < 0 { 
						y2 += usedwidth; 
					} 
					;
					
					colval = middlefunc(lt_val, rt_val, middleidx_val, noise.map[(y2 * noise_width + x) as usize]);
					colval += rng.gen_range(-seed, seed);
					noise.min = noise.min.min(colval);
					noise.max = noise.max.max(colval);
					y2 = y - middle;
					noise.map[(y2 * noise_width + x)  as usize] = colval;
					
					////down
					y2 = y + length;
					if y2 > usedwidth { 
						y2 -= usedwidth;
					}
					
					colval = middlefunc(lb_val, rb_val, middleidx_val, noise.map[(y2 * noise_width + x) as usize]);
					colval += rng.gen_range(-seed, seed);
					noise.min = noise.min.min(colval);
					noise.max = noise.max.max(colval);
					y2 = y + middle;
					noise.map[(y2 * noise_width + x) as usize] = colval;

					////left
					let mut x2: i32 = x - length;
					if x2 < 0 { 
						x2 += usedwidth;
					}
					
					colval = middlefunc(lt_val, lb_val, middleidx_val, noise.map[(y * noise_width + x2) as usize]);
					colval += rng.gen_range(-seed, seed);
					noise.min = noise.min.min(colval);
					noise.max = noise.max.max(colval);
					x2 = x - middle;
					noise.map[(y * noise_width + x2) as usize] = colval;

					////right
					x2 = x + length;
					if x2 > usedwidth {
						x2 -= usedwidth;
					}
					
					colval = middlefunc(rt_val, rb_val, middleidx_val , noise.map[(y * noise_width + x2) as usize]);
					colval += rng.gen_range(-seed, seed);
					noise.min = noise.min.min(colval);
					noise.max = noise.max.max(colval);
					x2 = x + middle;
					noise.map[(y * noise_width + x2) as usize] = colval;
					
				}
			}

		}
		
		if middle >= 2 {
			//OLD Version should set as seed reduction func param->seed = seed-reduction;
			//param->seed = seed * reduction;
			self.seed = seedreducefunc(seed, reduction);
			self.length = middle;
			self.create();
		}
		
	}
	
}

/*


void 
create_diamond_square(diamond_square_t * param){
	noise_t * noise = param->noise;
	int length = param->length;
	float startseed = param->startseed;
	float seed = param->seed;
	float reduction = param->reduction;
	float (*middlefunc)(float x1, float x2, float x3, float x4) = param->middlefunc;
	float (*seedreducefunc)(const float seed, const float reduction) = param->seedreducefunc;
	
	float * noise_array = (float *)noise->map->entries;
	int noise_width = noise->map->config->size;
	int maxidx = noise_width - 1;
	int usedwidth = maxidx;
	maxidx *= maxidx;
	int cntsquare = (noise_width - 1) / length;
	int middle = length >> 1;
	#ifdef debug
		printf("oldmiddle: %i, newmiddle: %i ,squares: %i\n", length, middle, cntsquare);
	#endif	
	float colval;
	if (cntsquare == 1){
		getnewcolor(noise, &startseed, &noise_array[0]);
		getnewcolor(noise, &startseed, &noise_array[length]);
		getnewcolor(noise, &startseed, &noise_array[length*length+length]);
		getnewcolor(noise, &startseed, &noise_array[length*length + (2*length)]);
	}
	#if 0
		//create all new middle points
	#endif
	for (int sqy = 0; sqy < cntsquare; ++sqy){
		int sqx = 0;
		for (; sqx < cntsquare; ++sqx){
			//for each square
			int lt = sqy * length * noise_width + (sqx*length);
			int rt = sqy * length * noise_width + ((sqx+1)*length);
			int lb = (sqy+1) * length * noise_width + (sqx*length);
			int rb = (sqy+1) * length * noise_width + ((sqx+1)*length);
			//middlepoint
			int x = (sqx*length) + middle;
			int y = (sqy*length) + middle;
			#if defined(debug) && debug == 2
				printf("sqx: %i, sqy: %i, x: %i, y: %i \n", sqx, sqy, x, y);
				printf("middelpoint: x: %i, y: %i \n", x, y);
			#endif
			//array_get_ref(noise_array,lt)
			colval = (noise_array[lt] + noise_array[rt] + noise_array[lb] + noise_array[rb])*0.25;
			colval += seedrndlh(-seed, seed);		
			noise->min = fminf(noise->min, colval);
			noise->max = fmaxf(noise->max, colval);
			noise_array[y * noise_width + x] = colval;
		}
	}	
	#if 0
		//create all new sidepoints
	#endif
	for (int sqy = 0; sqy < cntsquare; ++sqy){
		int sqx = 0;
		for (; sqx < cntsquare; ++sqx){
			//for each square			
			float lt_val = noise_array[sqy * length * noise_width + (sqx*length)];
			float rt_val = noise_array[sqy * length * noise_width + ((sqx+1)*length)];
			float lb_val = noise_array[(sqy+1) * length * noise_width + (sqx*length)];
			float rb_val = noise_array[(sqy+1) * length * noise_width + ((sqx+1)*length)];
			
			//middlepoint
			int x = (sqx*length) + middle;
			int y = (sqy*length) + middle;
			
			float middleidx_val = noise_array[y * noise_width + x];
			//sidepoints
			//up
			int y2 = y - length;
			if ( y2 < 0 ) { 
				y2 += usedwidth; 
			} 
			
			colval = (*middlefunc)(lt_val, rt_val, middleidx_val, noise_array[y2 * noise_width + x]);
			colval += seedrndlh(-seed, seed);
			noise->min = fminf(noise->min, colval);
			noise->max = fmaxf(noise->max, colval);
			y2 = y - middle;
			noise_array[y2 * noise_width + x] = colval;
			
			//down
			y2 = y + length;
			if (  y2 > usedwidth ) { 
				y2 -= usedwidth;
			}
			
			colval = (*middlefunc)(lb_val, rb_val, middleidx_val, noise_array[ y2 * noise_width + x]);
			colval += seedrndlh(-seed, seed);
			noise->min = fminf(noise->min, colval);
			noise->max = fmaxf(noise->max, colval);
			y2 = y + middle;
			noise_array[y2 * noise_width + x] = colval;

			//left
			int x2 = x - length;
			if (  x2 < 0 ) { 
				x2 += usedwidth;
			}
			
			colval = (*middlefunc)(lt_val, lb_val, middleidx_val, noise_array[y * noise_width + x2]);
			colval += seedrndlh(-seed, seed);
			noise->min = fminf(noise->min, colval);
			noise->max = fmaxf(noise->max, colval);
			x2 = x - middle;
			noise_array[y * noise_width + x2] = colval;

			//right
			x2 = x + length;
			if (  x2 > usedwidth ) {
				x2 -= usedwidth;
			}
			
			colval = (*middlefunc)(rt_val, rb_val, middleidx_val , noise_array[y * noise_width + x2]);
			colval += seedrndlh(-seed, seed);
			noise->min = fminf(noise->min, colval);
			noise->max = fmaxf(noise->max, colval);
			x2 = x + middle;
			noise_array[y * noise_width + x2] = colval;
		}
	}	
	if(middle >= 2){
		param->seed = seedreducefunc(seed, reduction);
		param->length = middle;
		create_diamond_square(param);
	}
}


*/