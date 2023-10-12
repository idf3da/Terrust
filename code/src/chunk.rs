use bevy::prelude::*;

use crate::{
        CHUNK_MAX_NUM,
};

use bevy::render::mesh::VertexAttributeValues;
use bevy::render::mesh::Indices;
use meshx::TriMesh;

pub struct Chunk {
        pub mesh: Mesh,
        pub display: bool,
}

#[derive(Default)]
pub struct ChunksResource {
        pub chunks: Vec<Chunk>
}

#[derive(Component)]
pub struct MeshIndicator {}

impl ChunksResource {
        // Despawn all chunks that exist in the world
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
                let chunk = &self.chunks[(int_pos.0 + int_pos.1 * CHUNK_MAX_NUM.1) as usize];
                let mesh = &chunk.mesh;

                println!("Save initiated.");
                
                // Extract indecies in Vec format and chunk them into triplets
                // This is required by the standard meshx library index notation
                if let Some(VertexAttributeValues::Float32x3(positions)) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) {
                        if let Some(indices) = mesh.indices() {
                                let stuff = indices.to_owned();
                                if let Indices::U32(vector) = stuff {
                                        let trimesh = TriMesh::new(positions.to_vec(), vec_chunk_triplets(vector));
                                        println!("{:?}", meshx::io::save_trimesh(&trimesh, file_name).unwrap());
                                        println!("Save successfull.")
                                }
                                
                        } else {
                                print!("Save failed Err #1.");
                        }
                } else {
                        print!("Save failed Err #2.");
                }
                
        }
}

// xxxxxx --> xxx, xxx
fn vec_chunk_triplets(vec: Vec<u32>) -> Vec<[usize; 3]> {
        let mut i = 0;
        let mut vec_res: Vec<[usize; 3]> = Vec::new();
        
        for _ in 0..(vec.len() as i32 / 3) {
                vec_res.push([vec[i as usize]  as usize, vec[(i + 1) as usize]  as usize, vec[(i + 2) as usize]  as usize]);
                i += 3;
        }

        return vec_res


}

// At the start fills the world with blank chunks
pub fn chunks_init(mut chunks: ResMut<ChunksResource>) {
        chunks.chunks = Vec::new();
        for _ in 0..CHUNK_MAX_NUM.0 * CHUNK_MAX_NUM.1 {
                chunks.chunks.push(Chunk{
                        mesh: Mesh::new(bevy::render::mesh::PrimitiveTopology::TriangleList),
                        display: false,
                });
        }
}



