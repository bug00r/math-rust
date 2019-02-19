use algorithm::noise::md::*;

#[test]
fn midpoint_displacement() {
	let w: i32 = 17;
	let h: i32 = w;
	let maxreduction: f32 = 1.;
	let reduction: f32 = 0.5;
	let mut md: MidpointDisplacement = MidpointDisplacement::new(w as u32, h as u32);
	md.length = w-1;
	md.startseed = 1.;
	md.seed = maxreduction;
	md.reduction = reduction;
	md.create();
	
	//let maxidx = w * w; 
	
	//print!("[");
	//for idx in 0..maxidx {
	//	if idx % w == 0 {
	//		print!("\n");
	//	}
	//	print!(" {:1.7}", md.noise.map[idx as usize]);
	//}
	//print!("]\n");
	
}