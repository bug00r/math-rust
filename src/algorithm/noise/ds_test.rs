use algorithm::noise::ds::*;

#[test]
fn diamond_square() {
	let w: i32 = 17;
	let h: i32 = w;
	let maxreduction: f32 = 1.;
	let reduction: f32 = 0.5;
	let mut ds: DiamondSquare = DiamondSquare::new(w as u32, h as u32);
	ds.length = w-1;
	ds.startseed = 1.;
	ds.seed = maxreduction;
	ds.reduction = reduction;
	ds.create();
	
	//let maxidx = w * w; 
	//
	//print!("[");
	//for idx in 0..maxidx {
	//	if idx % w == 0 {
	//		print!("\n");
	//	}
	//	print!(" {:1.7}", ds.noise.map[idx as usize]);
	//}
	//print!("]\n");
	
}