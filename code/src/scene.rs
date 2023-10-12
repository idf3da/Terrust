use bevy::prelude::*;
use crate::meshgen;

pub fn basic_scene(
	mut commands: Commands, 
	mut meshes: ResMut<Assets<Mesh>>, 
	asset_server: Res<AssetServer>, 
	mut materials: ResMut<Assets<StandardMaterial>>
) {
	// Global light
	commands.spawn_bundle(DirectionalLightBundle {
		directional_light: DirectionalLight {
			illuminance: 30000.0,
			..default()
		},
		transform: Transform::from_xyz(0.0, 0.0, 0.0)
			.with_rotation(Quat::from_rotation_x(-3.14 / 2.0)),
		..default()
	});

	let skybox_box = meshes.add(meshgen::gen_skybox());
	let texture_handle = asset_server.load("textures/sky.png");

	// Skybox material
	let material_handle = materials.add(StandardMaterial {
		base_color_texture: Some(texture_handle.clone()),
		alpha_mode: AlphaMode::Blend,
		unlit: true, // Does not cast shadows
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
			size: 10000000.0,
		})),
		material: materials.add(StandardMaterial {
			base_color: Color::rgba(0.35, 0.55, 0.85, 0.733),
			alpha_mode: AlphaMode::Blend, // Prevents clipping outside of terrain area
			unlit: true,
			..default()
		}),
		transform: Transform::from_xyz(0.0, 0.0, 0.0),
		..default()
	});
	
}