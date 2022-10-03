use bevy::prelude::*;
// use rand::Rng;
use noise::{NoiseFn, Perlin};
use bevy::render::mesh::VertexAttributeValues;

fn type_of<T>(_: T){
	println!("{}", std::any::type_name::<T>());
}


pub fn generate_grid(size: (u32, u32)) -> Mesh {

        println!("Generating grid {} by {} squares", size.0, size.1);

        let mut mesh = Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList);
        let pos = generate_verts(size);

        mesh.set_indices(Some(bevy::render::mesh::Indices::U32(generate_indicies(size))));
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, pos.clone());
        // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, generate_normals(size, pos.clone()));
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, generate_smooth_normals(size, pos.clone()));
        // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[1.0, 1.0]; ((size.0 + 1) * (size.1 + 1)) as usize]);

        return mesh;
}

fn generate_verts(size: (u32, u32)) -> Vec<[f32; 3]> {
	let mut vert: Vec<[f32; 3]> = Vec::new();
        let mut rng = rand::thread_rng();
        let perlin = Perlin::new(); // TODO: Rewrite cuz min and max are capped?
        
        let scale2d1st = 0.00050;
        let scaley1st = 350.0;

        let scale2d2nd = scale2d1st * 4.0; 
        let scaley2nd = scaley1st / 2.0;
        let weight2nd = 1.25;

        let scale2d3rd = scale2d2nd * 4.0;
        let scaley3rd = scaley2nd / 2.0;
        let weight3rd = 0.5;

        let scale2d4th = scale2d3rd * 4.0;
        let scaley4th = scaley3rd / 2.0;
        let weight4th = 0.0625;

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

        println!("Idx count: {}", idx.len());

        return idx;
}

fn generate_normals(size: (u32, u32), positions: Vec<[f32; 3]>) -> Vec<[f32; 3]> {
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut A: [f32; 3];
        let mut B: [f32; 3];
        let mut C: [f32; 3];
        let mut D: [f32; 3];

        let mut c: u32 = 0;


        for i in 0..size.0+1 {
                for j in 0..size.1+1 {
                        let mut A: [f32; 3] = [0.0, 0.0, 0.0];
                        let mut B: [f32; 3] = [0.0, 0.0, 0.0];
                        
                        // println!("{}x{}", i, j);

                        if j == size.1 {
                                A = vec_sub(positions[(i + j * size.0) as usize], positions[(i + j * size.0 - 1) as usize]); 
                        }
                        
                        if i == size.0 {
                                B = vec_sub(positions[(i + j * size.0) as usize], positions[(i + (j + 0) * size.0 + 1) as usize]);  
                        } else {
                                A = vec_sub(positions[(i + j * size.0) as usize], positions[(i + j * size.0 + 1) as usize]);
                                B = vec_sub(positions[(i + j * size.0) as usize], positions[(i + (j + 1) * size.0 + 1) as usize]);
                        }
                        
                        normals.push(cross_prod(A, B));
                        c += 1;
                }
        }


        println!("Normals: {}", normals.len());

        return normals;
}

fn generate_smooth_normals(size: (u32, u32), positions: Vec<[f32; 3]>) -> Vec<[f32; 3]> {
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut O: [f32; 3];
        let mut A: [f32; 3];
        let mut B: [f32; 3];
        let mut C: [f32; 3];
        let mut D: [f32; 3];

        let mut tr1: [f32; 3];

        let mut c: u32 = 0;
        
        
        for i in 0..size.0+1 {
                for j in 0..size.1+1 {                        
                        // println!("{}x{}", i, j);

                        O = positions[(i * (size.0 + 1) + j) as usize];

                        if i == 0 {
                                A = O
                        } else {
                                A = positions[((i - 1) * (size.0 + 1) + j) as usize];
                        }
        
                        if j == 0 {
                                D = O
                        } else {
                                D = positions[(i * (size.0 + 1) + j - 1) as usize];
                        }
                        
                        
                        if i == size.0 {
                                C = O
                        } else {                                
                                C = positions[((i + 1) * (size.0 + 1) + j) as usize];
                        }
                        
                        if j == size.1 {
                                B = O
                        } else {                                
                                B = positions[(i * (size.0 + 1) + j + 1) as usize];
                        }

                        
                        normals.push(vec_wt_avg(O, A, B, C, D));
                        c += 1;
                }
        }
        
        
        println!("Normals: {}", normals.len());
        
        return normals;
}

// TODO: fn generate_smooth_normals()
// https://computergraphics.stackexchange.com/questions/4031/programmatically-generating-vertex-normals

fn vec_sub(A: [f32; 3], B: [f32; 3]) -> [f32; 3] {
        return [B[0] - A[0], B[1] - A[1], B[2] - A[2]];        
}

fn vec_add(A: [f32; 3], B: [f32; 3]) -> [f32; 3] {
        return [B[0] + A[0], B[1] + A[1], B[2] + A[2]];        
}

fn cross_prod (A: [f32; 3], B: [f32; 3]) -> [f32; 3] {
        return [A[1]*B[2] - A[2]*B[1], A[2]*B[0]-A[0]*B[2], A[0]*B[1]-A[1]*B[0]];
}

fn vec_wt_avg(O: [f32; 3], A: [f32; 3], B: [f32; 3], C: [f32; 3], D: [f32; 3]) -> [f32; 3] {
        let mut final_vec: [f32; 3] = [0.0, 0.0, 0.0];

        let a = vec_norm(vec_sub(O, A));
        let b = vec_norm(vec_sub(O, B));
        let c = vec_norm(vec_sub(O, C));
        let d = vec_norm(vec_sub(O, D));
        
        let area1 = tr_area(a, b);
        let area2 = tr_area(b, c);
        let area3 = tr_area(c, d);
        let area4 = tr_area(d, a);

        let tr1 = cross_prod(A, B);
        let tr2 = cross_prod(B, C);
        let tr3 = cross_prod(C, D);
        let tr4 = cross_prod(D, A);

        final_vec = vec_add(final_vec, vec_scale(tr1, area1));
        final_vec = vec_add(final_vec, vec_scale(tr2, area2));
        final_vec = vec_add(final_vec, vec_scale(tr3, area3));
        final_vec = vec_add(final_vec, vec_scale(tr4, area4));

        final_vec = vec_scale(final_vec, 1.0/(area1 + area2 + area3 + area4));

        return final_vec
}

fn tr_area(A: [f32; 3], B: [f32; 3]) -> f32 {
        return 0.5*( (A[1]*B[2] - A[2]*B[1]).powi(2) + (A[2]*B[0] - A[0]*B[2]).powi(2) + (A[0]*B[1] - A[1]*B[0]).powi(2) ).powi(1/2);
}

fn vec_mag(V: [f32; 3]) -> f32 {
        return (V[0].powi(2) + V[1].powi(2) + V[2].powi(2)).powi(1/2).abs()
}

pub fn vec_scale(V: [f32; 3], S: f32) -> [f32; 3] {
        let mut v: [f32; 3] = [0.0, 0.0, 0.0];
        v[0] = V[0] * S;
        v[1] = V[1] * S;
        v[2] = v[2] * S;
        return v
}

pub fn inv_norm(V: std::option::Option<&VertexAttributeValues>) -> std::option::Option<&VertexAttributeValues> {
        let mut final_vec: [f32; 3];
        for i in V.iter() {
                final_vec.push(i.as_float3())
        }
}

fn vec_norm(V: [f32; 3]) -> [f32; 3] {
        return vec_scale(V, vec_mag(V));
}