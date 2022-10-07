use noise::{NoiseFn, Perlin};

pub fn gen_perlin(size: (u32, u32)) -> Vec<[f32; 3]> {
	let mut vert: Vec<[f32; 3]> = Vec::new();
        let perlin = Perlin::new(); // TODO: Rewrite cuz min and max are capped?
        
        let scale2d1st = 0.00050;
        let scaley1st = 350.0;

        let scale2d2nd = scale2d1st * 4.0; 
        let scaley2nd = scaley1st / 2.0;
        let weight2nd = 1.25;

        let scale2d3rd = scale2d2nd * 8.0;
        let scaley3rd = scaley2nd / 4.0;
        let weight3rd = 0.5;

        let scale2d4th = scale2d3rd * 32.0;
        let scaley4th = scaley3rd / 64.0;
        let weight4th = 0.125;

	for x in 0..size.0+1 {
		for z in 0..size.1+1 {

                        let y1 = perlin.get([(x as f64 * scale2d1st + 0.5), (z as f64 * scale2d1st + 0.5)]) * scaley1st;
                        let y2 = perlin.get([(x as f64 * scale2d2nd + 0.5), (z as f64 * scale2d2nd + 0.5)]) * scaley2nd * 2.0;
                        let y3 = perlin.get([(x as f64 * scale2d3rd + 0.5), (z as f64 * scale2d3rd + 0.5)]) * scaley3rd * 8.0;
                        let y4 = perlin.get([(x as f64 * scale2d4th + 0.5), (z as f64 * scale2d4th + 0.5)]) * scaley4th * 16.0;  

                        let finaly = (y1 + y2 * weight2nd + y3 * weight3rd + y4 * weight4th) / 4.0;
			vert.push([x as f32, finaly as f32, z as f32]);
		}
	}

        println!("Generated {} by {} vertecies", (0..size.0+1).len(), (0..size.1+1).len());
        println!("Vertex count: {}", vert.len());

	return vert;
}




