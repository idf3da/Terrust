use bevy::{
        prelude::*, 
        ecs::system::Resource
};

use crate::meshgen;
use crate::{
        CHUNK_SIZE,
        SCALE,
        CHUNK_NUM
};

pub struct Chunk {
        mesh: Mesh,
        visible: bool,
        display: bool,
}

#[derive(Default)]
pub struct ChunksResource {
        pub chunks: Vec<Chunk>
}

pub fn chunks_init(mut chunks: ResMut<ChunksResource>) {
        chunks.chunks = Vec::new();
        for _ in 0..CHUNK_NUM.0 * CHUNK_NUM.1 {
                chunks.chunks.push(Chunk{
                        mesh: Mesh::from(shape::Icosphere {
                                radius: 10.0,
                                subdivisions: 2,
                        }),
                        visible: true,
                        display: false,
                });
        }
}

pub fn chunks_update_sys(
        mut meshes: ResMut<Assets<Mesh>>,
        mut chunks: ResMut<ChunksResource>,
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut camera: Query<&mut Transform, With<Camera>>,
) {
        for cam in camera.iter_mut() {
                let pos: (f32, f32) = (cam.translation[0], cam.translation[2]);

                let mut coords: (u32, u32) = (0, 0);
                if pos.0 < CHUNK_NUM.0 as f32 * SCALE {
                        coords.0 = (pos.0 / SCALE) as u32;
                }
                if pos.1 < CHUNK_NUM.1 as f32 * SCALE {
                        coords.1 = (pos.1 / SCALE) as u32
                }
                if pos.0 < 0.0 {coords.0 = 0};
                if pos.1 < 0.0 {coords.1 = 0};

                let idx = (coords.0 + coords.1 * CHUNK_NUM.1) as usize;
                println!("pos: {:?}, coords: {:?}, chunk: {}", pos, coords, coords.0 + coords.1 * CHUNK_NUM.1);
                // TODO: check generated chunks in range, use chunks_load_near()
                if chunks.chunks[idx].display == false {
                        commands.spawn_bundle(PbrBundle {
                                mesh: meshes.add(meshgen::gen_ter_mesh(CHUNK_SIZE, (pos.0, pos.1))),
                                // mesh: meshes.add(Mesh::from(shape::Icosphere {
                                //         radius: 10.0,
                                //         subdivisions: 2,
                                // })),
                                transform: Transform::from_scale(Vec3::splat(SCALE / CHUNK_SIZE.0 as f32)).with_translation(Vec3{x: coords.0 as f32 * SCALE, y: 0.0, z: coords.1 as f32 * SCALE}),
                                material: materials.add(StandardMaterial {
                                        base_color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                                        perceptual_roughness: 1.0,
                                        metallic: 0.0,
                                        ..default()
                                }),
                                ..Default::default()
                        });
                        chunks.chunks[idx].display = true;
                }
                
        }
}

fn chunks_load_near(pos: (u32, u32), rad: u32)  -> Vec<[u32; 2]>  {
        let mut res: Vec<[u32; 2]> = Vec::new();

        for i in 0..rad {
                for j in 0..rad {
                        res.push([pos.0 + i, pos.1 + j])
                }
        }

        return res
}