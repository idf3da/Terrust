mod terrain;

use bevy::{
	pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
	prelude::*,
	render::{render_resource::WgpuFeatures, settings::WgpuSettings},
};
use bevy_flycam::PlayerPlugin;

const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);

fn main() {
	App::new()
		.insert_resource(ClearColor(CLEAR))
		.insert_resource(WindowDescriptor {
			height: 800.0,
			width: 600.0,
			title: "3D".to_string(),
			..default()
		})
		.insert_resource(WgpuSettings {
			features: WgpuFeatures::POLYGON_MODE_POINT,
			..default()
		})
		.add_plugins(DefaultPlugins)
		.add_plugin(WireframePlugin)
		.add_plugin(PlayerPlugin)
		.add_startup_system(setup)
		.run();
}

/// set up a simple 3D scene
fn setup(
	mut commands: Commands,
	mut wireframe_config: ResMut<WireframeConfig>,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	wireframe_config.global = false;


	commands.spawn_bundle(PointLightBundle {
		transform: Transform::from_translation(Vec3::new(32.0, 16.0, 32.0)),
		..Default::default()
	});

	commands
		.spawn_bundle(PbrBundle {
			mesh: meshes.add(terrain::generate_grid((64, 64))),
			material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
			..Default::default()
		})
		.insert(Wireframe);
}