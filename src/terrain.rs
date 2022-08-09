use bevy::prelude::*;
use rand::Rng;

pub fn generate_grid(size: (u32, u32)) -> Mesh {
        let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList);

        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(generate_indicies(size))));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, generate_positions(size));
mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL,vec![[0.0, 1.0, 0.0]; ((size.0 + 1) * (size.1 + 1)) as usize]);
        // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[1.0, 1.0]; ((size.0 + 1) * (size.1 + 1)) as usize]);

        return mesh;
}

fn generate_positions(size: (u32, u32)) -> Vec<[f32; 3]> {
	let mut vert = Vec::new();
        let mut rng = rand::thread_rng();

	for x in 0..size.0+1 {
		for z in 0..size.1+1 {
			// vert.push([x as f32, 0.0, z as f32]);
                        vert.push([x as f32, rng.gen_range(-1.0..3.0), z as f32]);
		}
	}

	return vert;
}

fn generate_indicies(size: (u32, u32)) -> Vec<u32> {
	let mut idx: Vec<u32> = vec![0 as u32; (size.0 * size.1 * 6) as usize];

	let mut vert: u32 = 0;
	let mut tris: usize = 0;


	for _ in 0..size.0 {
		for _ in 0..size.1 {
                        idx[tris + 0] = vert;
                        idx[tris + 1] = vert + 1;
                        idx[tris + 2] = vert + size.1 + 1;

                        idx[tris + 3] = vert + 1;
                        idx[tris + 4] = vert + size.1 + 2;
                        idx[tris + 5] = vert + size.1 + 1;
                        
                        vert += 1;
                        tris += 6;
                }

                vert += 1
        }

        return idx;
}
