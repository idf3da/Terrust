mod terrain;
mod meshgen;
mod mathop;
mod scene;
mod systems;
mod chunk;

use crate::scene::*;
use crate::systems::*;
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
use bevy_flycam::{PlayerPlugin, MovementSettings};


const S: u32 = 5;
const CHUNK_SIZE: (u32, u32) = (2 << S, 2 << S);
const CHUNK_NUM: (u32, u32) = (32, 32);
const SCALE: f32 = 1000.0;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.insert_resource(WindowDescriptor {
			height: 800.0,
			width: 600.0,
			title: "Terrust 1.0".to_string(),
			..default()
		})
		.insert_resource(WgpuSettings {
			features: WgpuFeatures::POLYGON_MODE_LINE,
			..default()
		})
		.add_plugin(WireframePlugin)
		.init_resource::<ChunksResource>()
		.add_plugin(PlayerPlugin)
		.insert_resource(MovementSettings {
			sensitivity: 0.0002,
			speed: 512.0,
		})
		// .add_startup_system(spawn_viewport)
		.add_startup_system(basic_scene)
		.add_startup_system(setup)
		.add_startup_system(chunks_init)
		.add_system(move_camera)
		.add_system(chunks_update_sys)
		.run();
}

//TODO: Chunk array accesible from everywhere

fn setup(
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

	


}