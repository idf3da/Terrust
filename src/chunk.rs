use bevy::ecs::system::EntityCommands;
use bevy::{
        prelude::*, 
        ecs::system::Resource
};
use bevy::prelude::*;

use crate::meshgen;
use crate::ui::*;
use crate::{
        CHUNK_RES,
        SCALE,
        CHUNK_MAX_NUM,
        RENDER_DISTANCE
};

use bevy::render::mesh::VertexAttributeValues;
use bevy::render::mesh::Indices;
use meshx::TriMesh;

pub struct Chunk {
        mesh: Mesh,
        visible: bool,
        display: bool,
}

#[derive(Default)]
pub struct ChunksResource {
        pub chunks: Vec<Chunk>
}

#[derive(Component)]
pub struct MeshIndicator {

}

impl ChunksResource {
        pub fn flush (
                &mut self,
                mut query: Query<Entity, With<MeshIndicator>>, 
                mut commands: Commands,
        ) {
                query.for_each(|entity| {
                        commands.entity(entity).despawn();
                });

                for i in 0..CHUNK_MAX_NUM.0 * CHUNK_MAX_NUM.1 {
                        self.chunks[i as usize].display = false;
                }
                
                print!("Flushed.")
        }

        pub fn save (
                &mut self,
                int_pos: (u32, u32),
                file_name: String
        ) {
                // TODO: Pass this to IO
                let chunk = &self.chunks[(int_pos.0 + int_pos.1 * CHUNK_MAX_NUM.1) as usize];
                let mesh = &chunk.mesh;

                println!("Save initiated.");
        
                println!("{:?}", (int_pos.0 + int_pos.1 * CHUNK_MAX_NUM.1) as usize);
                
                if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
                        if let Some(indices) = mesh.indices() {
                                let stuff = indices.to_owned();
                                if let Indices::U32(vector) = stuff {
                                        let trimesh = TriMesh::new(positions.to_vec(), vec_chunk(vector));
                                        println!("{:?}", meshx::io::save_trimesh(&trimesh, file_name).unwrap());
                                        println!("Save successfull?")
                                }
                                
                        } else {
                                print!("Save failed #1.");
                        }
                } else {
                        print!("Save failed #2.");
                }
                
        }
}

fn vec_chunk(vec: Vec<u32>) -> Vec<[usize; 3]> {
        let mut i = 0;
        let mut vec_res: Vec<[usize; 3]> = Vec::new();
        
        for _ in 0..(vec.len() as i32 / 3) {
                vec_res.push([vec[i as usize]  as usize, vec[(i + 1) as usize]  as usize, vec[(i + 2) as usize]  as usize]);
                i += 3;
        }

        return vec_res


}

fn generate_indicies(size: (u32, u32)) -> Vec<[usize; 3]> {
	let mut idx = Vec::new();

	let mut vert: u32 = 0;
	let mut tris: usize = 0;

        let mut ind1 = [0 as usize; 3];
        let mut ind2 = [0 as usize; 3];

	for _ in 0..size.0 {
		for _ in 0..size.1 {
                        ind1[0] = vert as usize;
                        ind1[1] = (vert + 1) as usize;
                        ind1[2] = (vert + size.1 + 1) as usize;
                        
                        ind2[0] = (vert + size.1 + 2) as usize;
                        ind2[1] = (vert + size.1 + 1) as usize;
                        ind2[2] = (vert + 1) as usize;

                        idx.push(ind1);
                        idx.push(ind2);
                        
                        vert += 1;
                        tris += 6;
                }

                vert += 1
        }

        return idx;
}

pub fn chunks_init(mut chunks: ResMut<ChunksResource>) {
        chunks.chunks = Vec::new();
        for _ in 0..CHUNK_MAX_NUM.0 * CHUNK_MAX_NUM.1 {
                chunks.chunks.push(Chunk{
                        mesh: Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList),
                        visible: true,
                        display: false,
                });
        }
}

pub fn chunks_update_sys (
        mut meshes: ResMut<Assets<Mesh>>,
        mut chunks: ResMut<ChunksResource>,
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut ui_state: ResMut<UiState>,
        mut camera: Query<&mut Transform, With<Camera>>,
) {
        for cam in camera.iter_mut() {
                let cam_position: (f32, f32) = (cam.translation[0], cam.translation[2]);

                let mut int_coordinates: (u32, u32) = (0, 0);

                if cam_position.0 < CHUNK_MAX_NUM.0 as f32 * SCALE {
                        int_coordinates.0 = (cam_position.0 / SCALE) as u32;
                }
                if cam_position.1 < CHUNK_MAX_NUM.1 as f32 * SCALE {
                        int_coordinates.1 = (cam_position.1 / SCALE) as u32
                }

                if cam_position.0 < 0.0 {int_coordinates.0 = 0};
                if cam_position.1 < 0.0 {int_coordinates.1 = 0};

                // Update UI
                ui_state.int_location = int_coordinates;

                // Render near chunk
                for near_int_coordinates in chunks_load_near(int_coordinates, ui_state.render_distance) {
                        let idx = (near_int_coordinates.0 + near_int_coordinates.1 * CHUNK_MAX_NUM.1 as i32) as usize;
                        if chunks.chunks[idx].display == false {
                                let ChunkMesh = meshgen::gen_ter_mesh(CHUNK_RES, (near_int_coordinates.0 as f32 * CHUNK_RES.0 as f32, near_int_coordinates.1 as f32 * CHUNK_RES.1 as f32));
                                commands.spawn_bundle(PbrBundle {
                                        mesh: meshes.add(ChunkMesh.clone()),
                                        transform: Transform::from_scale(Vec3::splat(SCALE / CHUNK_RES.0 as f32)).with_translation(Vec3{x: near_int_coordinates.0 as f32 * SCALE, y: 0.0, z: near_int_coordinates.1 as f32 * SCALE}),
                                        material: materials.add(StandardMaterial {
                                                base_color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                                                perceptual_roughness: 1.0,
                                                metallic: 0.0,
                                                ..default()
                                        }),
                                        ..Default::default()
                                }).insert(MeshIndicator{});
                                chunks.chunks[idx].display = true;
                                chunks.chunks[idx].mesh = ChunkMesh;
                        }

                }
                
        }
}

fn chunks_load_near(pos: (u32, u32), rad: u32)  -> Vec<(i32, i32)> {
        let mut res: Vec<(i32, i32)> = Vec::new();

        let posi = (pos.0 as i32, pos.1 as i32);

        for i in -(rad as i32)..(rad as i32) {
                for j in -(rad as i32)..(rad as i32) {
                        let mut x = 0;
                        if posi.0 + i > 0 {
                                x = posi.0 + i
                        }
                        if x > CHUNK_MAX_NUM.0 as i32 {
                                x = CHUNK_MAX_NUM.0 as i32
                        }

                        let mut y = 0;
                        if posi.1 + j > 0 {
                                y = posi.1 + j
                        }
                        if y > CHUNK_MAX_NUM.1 as i32 {
                                y = CHUNK_MAX_NUM.1 as i32
                        }

                        res.push((x, y))
                }
        }

        return res
}