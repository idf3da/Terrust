use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;

use crate::terrain::*;
use crate::mathop::*;



pub fn gen_skybox() -> bevy::prelude::Mesh {
	let mut skybox_mesh = Mesh::from(shape::UVSphere {
		radius: 1000000.0,
		sectors: 16,
		stacks: 8,
	});
	
	let idx_mesh = (*skybox_mesh.indices().unwrap()).iter().collect::<Vec<_>>();
	skybox_mesh.set_indices(Some(bevy::render::mesh::Indices::U32(inv_idx(idx_mesh))));

	return skybox_mesh
}

pub fn gen_ter_mesh(size: (u32, u32), pos: (f32, f32)) -> Mesh {

        let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList);
        let pos = gen_perlin(size, pos);
        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(generate_indicies(size))));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, pos.clone());
        // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, generate_normals(size, pos.clone()));
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, generate_smooth_normals(size, pos.clone()));

        if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
                let colors: Vec<[f32; 4]> = positions
			.iter()
			.map(|[x, y, z] | color_height(*x, *y, *z))
			.collect();
		mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
	}

        return mesh;
}

fn color_height(_x: f32, y: f32, _: f32) -> [f32; 4] {
        let mut c = [0.5, 0.5, 0.5, 1.0];

        if y > 50.0 {
                c = [1.0, 1.0, 1.0, 1.0]
        } else if y > 10.0 {
                c = [0.3, 1.0, 0.3, 1.0]       
        } else if y > 0.0 {
                c = [0.76, 0.7, 0.5, 1.0]
        } else {
                c = [0.5, 0.5, 0.5, 1.0]
        }



        return c
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

                        idx[tris + 3] = vert + size.1 + 2;
                        idx[tris + 4] = vert + size.1 + 1;
                        idx[tris + 5] = vert + 1;
                        
                        vert += 1;
                        tris += 6;
                }

                vert += 1
        }

        return idx;
}

fn generate_normals(size: (u32, u32), positions: Vec<[f32; 3]>) -> Vec<[f32; 3]> {
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut a: [f32; 3];
        let mut b: [f32; 3];


        for i in 0..size.0+1 {
                for j in 0..size.1+1 {
                        a = [0.0, 0.0, 0.0];
                

                        if j == size.1 {
                                a = vec_sub(positions[(i + j * size.0) as usize], positions[(i + j * size.0 - 1) as usize]); 
                        }
                        
                        if i == size.0 {
                                b = vec_sub(positions[(i + j * size.0) as usize], positions[(i + (j + 0) * size.0 + 1) as usize]);  
                        } else {
                                a = vec_sub(positions[(i + j * size.0) as usize], positions[(i + j * size.0 + 1) as usize]);
                                b = vec_sub(positions[(i + j * size.0) as usize], positions[(i + (j + 1) * size.0 + 1) as usize]);
                        }
                        
                        normals.push(cross_prod(a, b));
                }
        }


        return normals;
}

fn generate_smooth_normals(size: (u32, u32), positions: Vec<[f32; 3]>) -> Vec<[f32; 3]> {
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut o: [f32; 3];
        let mut a: [f32; 3];
        let mut b: [f32; 3];
        let mut c: [f32; 3];
        let mut d: [f32; 3];
        
        
        for i in 0..size.0+1 {
                for j in 0..size.1+1 {                        

                        o = positions[(i * (size.0 + 1) + j) as usize];

                        if i == 0 {
                                a = o
                        } else {
                                a = positions[((i - 1) * (size.0 + 1) + j) as usize];
                        }
        
                        if j == 0 {
                                d = o
                        } else {
                                d = positions[(i * (size.0 + 1) + j - 1) as usize];
                        }
                        
                        
                        if i == size.0 {
                                c = o
                        } else {                                
                                c = positions[((i + 1) * (size.0 + 1) + j) as usize];
                        }
                        
                        if j == size.1 {
                                b = o
                        } else {                                
                                b = positions[(i * (size.0 + 1) + j + 1) as usize];
                        }

                        
                        normals.push(tr_wt_avg(o, a, b, c, d));
                }
        }
        
        
        // println!("Normals: {}", normals.len());
        
        return normals;
}

pub fn inv_idx(idx: Vec<usize>) -> Vec<u32> {
        let mut idx_new: Vec<u32> = vec![0 as u32; idx.len()];
        for i in (0..(idx.len() - 3)).step_by(3) {
                idx_new[i] = idx[i + 2] as u32;
                idx_new[i + 1] = idx[i + 1] as u32;
                idx_new[i + 2] = idx[i] as u32;
        }

        return idx_new
}