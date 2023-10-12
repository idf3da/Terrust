use bevy::prelude::*;
use crate::chunk::*;

use crate::meshgen;
use crate::ui::*;
use crate::{
        CHUNK_RES,
        SCALE,
        CHUNK_MAX_NUM,
};

// Flush all chunks every 60 secconds
pub fn chunk_timed_flush(
        time: Res<Time>,
        mut query: Query<Entity, With<MeshIndicator>>, 
        mut commands: Commands,
        mut chunks: ResMut<ChunksResource>,
) {
        if time.seconds_since_startup() as i32 % 60 == 0 {
                chunks.flush(query, commands)
        }
}

// Scans the surroundings for unloaded chunks and generates them
pub fn chunks_update_sys (
        mut meshes: ResMut<Assets<Mesh>>,
        mut chunks: ResMut<ChunksResource>,
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut ui_state: ResMut<UiState>,
        mut camera: Query<&mut Transform, With<Camera>>,
) {
        for cam in camera.iter_mut() {
                // Convert camera position to world integer positon
                let cam_position: (f32, f32) = (cam.translation[0], cam.translation[2]);

                let mut int_coordinates: (u32, u32) = (0, 0);

                // <0 edge cases
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

                // Check each chunk within the render distance to see if its loaded
                for near_int_coordinates in chunks_load_near(int_coordinates, ui_state.render_distance) {
                        let idx = (near_int_coordinates.0 + near_int_coordinates.1 * CHUNK_MAX_NUM.1 as i32) as usize;
                        if chunks.chunks[idx].display == false { // If not loaded generate it
                                let ChunkMesh = meshgen::gen_ter_mesh(CHUNK_RES, (near_int_coordinates.0 as f32 * CHUNK_RES.0 as f32, near_int_coordinates.1 as f32 * CHUNK_RES.1 as f32), ui_state.layers.clone());
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
                                }).insert(MeshIndicator{}); // Mesh indicator for marking to use in despawn later
                                chunks.chunks[idx].display = true;
                                chunks.chunks[idx].mesh = ChunkMesh; // Saved for export function
                        }

                }
                
        }
}

// Given (X, Y) generates all chinks within rad radius in manhattan
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