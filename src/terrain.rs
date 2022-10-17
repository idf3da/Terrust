use noise::{NoiseFn, Perlin};

pub fn gen_perlin(size: (u32, u32), offset: (f32, f32)) -> Vec<[f32; 3]> {
	let mut vert: Vec<[f32; 3]> = Vec::new();
        let perlin = Perlin::new(); // TODO: Rewrite cuz min and max are capped?
        
        let freq = 0.001953125/2.0;
        let scale = 64.0;


	for x in 0..size.0+1 {
		for z in 0..size.1+1 {
                        let mut finally: f64 = 0.0;

                        finally += perlin.get([(x as f64 * freq * 1.0+ offset.0 as f64), (z as f64 * freq * 1.0+ offset.1 as f64)]) * scale / 1.0;
                        finally += perlin.get([(x as f64 * freq * 2.0+ offset.0 as f64), (z as f64 * freq * 2.0+ offset.1 as f64)]) * scale / 2.0;
                        finally += perlin.get([(x as f64 * freq * 4.0+ offset.0 as f64), (z as f64 * freq * 4.0+ offset.1 as f64)]) * scale / 4.0;
                        finally += perlin.get([(x as f64 * freq * 8.0+ offset.0 as f64), (z as f64 * freq * 8.0+ offset.1 as f64)]) * scale / 8.0;  
                        finally += perlin.get([(x as f64 * freq * 16.0+ offset.0 as f64), (z as f64 * freq * 16.0+ offset.1 as f64)]) * scale / 16.0;
                        finally += perlin.get([(x as f64 * freq * 32.0+ offset.0 as f64), (z as f64 * freq * 32.0+ offset.1 as f64)]) * scale / 32.0;  
                        finally += perlin.get([(x as f64 * freq * 64.0+ offset.0 as f64), (z as f64 * freq * 64.0+ offset.1 as f64)]) * scale / 64.0;  

			vert.push([x as f32, finally as f32, z as f32]);
		}
	}

        println!("Generated {} by {} vertecies", (0..size.0+1).len(), (0..size.1+1).len());
        println!("Vertex count: {}", vert.len());

	return vert;
}




