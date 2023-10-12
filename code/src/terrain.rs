use noise::{NoiseFn, Perlin};
use crate::{
        PERLIN_FREQ,
        PERLIN_HEIGHT_SCALE
};

// Generates an array of layered perlin noise according to parameters given in "layers"
pub fn gen_perlin(size: (u32, u32), offset: (f32, f32), layers: Vec<[i32; 3]>) -> Vec<[f32; 3]> {
	let mut vert: Vec<[f32; 3]> = Vec::new();
        let perlin = Perlin::new();

	for x in 0..size.0+1 {
		for z in 0..size.1+1 {
                        let mut finally: f64 = 0.0;
                        
                        // Cycle through layers vector that has [frequency, amplitude, is_enabled]
                        for l in layers.clone() {
                                if l[2] == 1 {
                                        finally += perlin.get([((x as f64 + offset.0 as f64) * PERLIN_FREQ * l[0] as f64), ((z as f64 + offset.1 as f64) * PERLIN_FREQ * l[0] as f64)]) * PERLIN_HEIGHT_SCALE * l[1] as f64;
                                }
                        }

			vert.push([x as f32, finally as f32, z as f32]);
		}
	}

	return vert;
}




