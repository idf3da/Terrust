mod terrain;
mod meshgen;
mod mathop;
mod scene;
mod systems;
mod chunk;
mod ui;

use crate::scene::*;
use crate::systems::*;
use crate::ui::*;
use crate::chunk::*;

use bevy::{
	pbr::wireframe::{WireframeConfig, WireframePlugin},
	prelude::*,
	render::{
		render_resource::{
			WgpuFeatures,
		}, 
		settings::WgpuSettings,
	},
};
use bevy_egui::{egui, EguiContext};
use bevy::{prelude::*, render::camera::Projection};
use bevy_flycam::{PlayerPlugin, MovementSettings};
use bevy_egui::{EguiPlugin};

const CAMERA_TARGET: Vec3 = Vec3::ZERO;
struct OriginalCameraTransform(Transform);

const S: u32 = 8;
const CHUNK_RES: (u32, u32) = (2 << S, 2 << S);
const CHUNK_MAX_NUM: (u32, u32) = (64, 64);
const SCALE: f32 = (S * S * 100) as f32;
const RENDER_DISTANCE: u32 = 16;
const PERLIN_HEIGHT_SCALE: f64 = 40.0;
const PERLIN_FREQ: f64 = 0.0009765625 * 8.0;

fn main() {
	println!("Starting!");
	App::new()
		.add_plugins(DefaultPlugins)
		.insert_resource(WindowDescriptor {
			height: 800.0,
			width: 600.0,
			title: "Terrust 1.4`".to_string(),
			..default()
		})
		.add_plugin(EguiPlugin)
		.add_system(ui_example_system)
		.init_resource::<UiState>()
		// .init_resource::<OccupiedScreenSpace>()
        	// .add_system(update_camera_transform_system)
		.insert_resource(WgpuSettings {
			features: WgpuFeatures::POLYGON_MODE_LINE,
			..default()
		})
		.add_plugin(WireframePlugin)
		.init_resource::<ChunksResource>()
		.add_plugin(PlayerPlugin)
		.insert_resource(MovementSettings {
			sensitivity: 0.0002,
			speed: 1400.0,
		})
		// .add_startup_system(spawn_viewport)
		.add_startup_system(basic_scene)
		.add_startup_system(setup)
		.add_startup_system(chunks_init)
		// .add_system(move_camera)
		.add_system(chunks_update_sys)
		.add_system(chunk_timed_flush)
		.run();
}

fn setup (
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut wireframe_config: ResMut<WireframeConfig>,
) {

	wireframe_config.global = false;

	// Reference sphere
	commands.spawn_bundle(PbrBundle {
		transform: Transform::from_translation(Vec3::new(SCALE, 0.0,SCALE)),
		mesh: meshes.add(Mesh::from(shape::Icosphere {
			radius: 10.0,
			subdivisions: 2,
		})),
		..Default::default()
	});

	let camera_pos = Vec3::new(-2.0, 2.5, 5.0);
	let camera_transform = Transform::from_translation(camera_pos).looking_at(CAMERA_TARGET, Vec3::Y);
	
	commands.insert_resource(OriginalCameraTransform(camera_transform));

}