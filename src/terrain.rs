use noise::{NoiseFn, Perlin};
use crate::{
        PERLIN_FREQ,
        PERLIN_HEIGHT_SCALE
};

pub fn gen_perlin(size: (u32, u32), offset: (f32, f32)) -> Vec<[f32; 3]> {
	let mut vert: Vec<[f32; 3]> = Vec::new();
        let perlin = Perlin::new();
        

	for x in 0..size.0+1 {
		for z in 0..size.1+1 {
                        let mut finally: f64 = 0.0;

                        finally += perlin.get([((x as f64 + offset.0 as f64) * PERLIN_FREQ * 1.0 as f64), ((z as f64 + offset.1 as f64) * PERLIN_FREQ * 1.0)]) * PERLIN_HEIGHT_SCALE / 1.0;
                        finally += perlin.get([((x as f64 + offset.0 as f64) * PERLIN_FREQ * 2.0 as f64), ((z as f64 + offset.1 as f64) * PERLIN_FREQ * 2.0)]) * PERLIN_HEIGHT_SCALE / 2.0;
                        finally += perlin.get([((x as f64 + offset.0 as f64) * PERLIN_FREQ * 4.0 as f64), ((z as f64 + offset.1 as f64) * PERLIN_FREQ * 4.0)]) * PERLIN_HEIGHT_SCALE / 4.0;
                        finally += perlin.get([((x as f64 + offset.0 as f64) * PERLIN_FREQ * 8.0 as f64), ((z as f64 + offset.1 as f64) * PERLIN_FREQ * 8.0)]) * PERLIN_HEIGHT_SCALE / 8.0;  
                        finally += perlin.get([((x as f64 + offset.0 as f64) * PERLIN_FREQ * 16.0 as f64), ((z as f64 + offset.1 as f64) * PERLIN_FREQ * 16.0)]) * PERLIN_HEIGHT_SCALE / 16.0;
                        finally += perlin.get([((x as f64 + offset.0 as f64) * PERLIN_FREQ * 32.0 as f64), ((z as f64 + offset.1 as f64) * PERLIN_FREQ * 32.0)]) * PERLIN_HEIGHT_SCALE / 32.0;  
                        finally += perlin.get([((x as f64 + offset.0 as f64) * PERLIN_FREQ * 64.0 as f64), ((z as f64 + offset.1 as f64) * PERLIN_FREQ * 64.0)]) * PERLIN_HEIGHT_SCALE / 64.0;  

			vert.push([x as f32, finally as f32, z as f32]);
		}
	}

        // println!("Generated {} by {} vertecies", (0..size.0+1).len(), (0..size.1+1).len());
        // println!("Vertex count: {}", vert.len());

	return vert;
}




