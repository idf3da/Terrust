mod terrain;
mod meshgen;
mod mathop;

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


const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
const S: u32 = 8;
const SIZE: (u32, u32) = (2 << S, 2 << S);
// const SIZE: (u32, u32) = (2, 2);

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.insert_resource(ClearColor(CLEAR))
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
		.add_plugin(PlayerPlugin)
		.insert_resource(MovementSettings {
			sensitivity: 0.0002,
			speed: 4096.0,
		})
		// .add_system(fade_transparency)
		.add_startup_system(setup)
		.run();
}

fn setup(
	mut commands: Commands,
	mut wireframe_config: ResMut<WireframeConfig>,
	mut meshes: ResMut<Assets<Mesh>>,
	asset_server: Res<AssetServer>,
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	let texture_handle = asset_server.load("textures/sky.png");
	wireframe_config.global = false;

	// Global light
	commands.spawn_bundle(DirectionalLightBundle {
		directional_light: DirectionalLight {
			illuminance: 30000.0,
			..default()
		},
		transform: Transform::from_xyz(40000.0, 2000.0, 40000.0)
			.with_rotation(Quat::from_rotation_x(-3.14 / 4.)),
		..default()
	});

	
	// Terrain mesh
	commands.spawn_bundle(PbrBundle {
		transform: Transform::from_scale(Vec3::splat(100.0)),
		mesh: meshes.add(meshgen::gen_ter_mesh(SIZE)),

		material: materials.add(StandardMaterial {
			base_color: Color::rgb(1.0, 1.0, 1.0),
			metallic: 0.5,
			..default()
		}),
		..Default::default()
	});

	// Reference sphere
	commands.spawn_bundle(PbrBundle {
		transform: Transform::from_translation(Vec3::new(40000.0, 2000.0,40000.0)),
		mesh: meshes.add(Mesh::from(shape::Icosphere {
			radius: 2000.0,
			subdivisions: 3,
		})),
		..Default::default()
	});

	let skybox_box = meshes.add(meshgen::gen_skybox());

	// Skybox material
	let material_handle = materials.add(StandardMaterial {
		base_color_texture: Some(texture_handle.clone()),
		alpha_mode: AlphaMode::Blend,
		unlit: true,
		..default()
	});

	// Skybox
	commands.spawn_bundle(PbrBundle {
		mesh: skybox_box.clone(),
		transform: Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(Quat::from_rotation_x(std::f32::consts::PI / 2.0)),
		material: material_handle,
		..default()
	});

	// Water plane
	commands.spawn_bundle(PbrBundle {
		mesh: meshes.add(Mesh::from(shape::Plane {
			size: 1000000.0,
		})),
		material: materials.add(StandardMaterial {
			base_color: Color::rgba(0.35, 0.55, 0.95, 0.7),
		
			alpha_mode: AlphaMode::Blend,
			unlit: true,
			..default()
		}),
		transform: Transform::from_xyz(10000.0, -7000.0, 15000.0),
		..default()
	});
	


}

// pub fn fade_transparency(time: Res<Time>, mut materials: ResMut<Assets<StandardMaterial>>) {
// 	let alpha = (time.time_since_startup().as_secs_f32().sin() / 2.0) + 0.5;
// 	for (_, material) in materials.iter_mut() {
// 		material.base_color.set_a(alpha);
// 	}
// }