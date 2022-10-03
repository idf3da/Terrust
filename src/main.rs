mod terrain;

use bevy::{
	pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
	pbr::{MaterialPipeline, MaterialPipelineKey},
	prelude::*,
	render::{
		mesh::MeshVertexBufferLayout,
		render_resource::{
			WgpuFeatures,
			RenderPipelineDescriptor,
			ShaderRef,
		}, 
		settings::WgpuSettings,
	},
};
use bevy_flycam::{PlayerPlugin, MovementSettings};
use bevy_atmosphere::prelude::*;


const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
const SUN: Color = Color::rgb(0.992156, 0.721568, 0.074509);
const S: u32 = 5;
const SIZE: (u32, u32) = (2 << S, 2 << S);
// const SIZE: (u32, u32) = (2, 2);
const HALF_SIZE: f32 = 1.0;

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		// .add_plugins(AtmospherePlugin)
		.insert_resource(ClearColor(CLEAR))
		.insert_resource(WindowDescriptor {
			height: 800.0,
			width: 600.0,
			title: "3D".to_string(),
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

	commands.spawn_bundle(DirectionalLightBundle {
		directional_light: DirectionalLight {
			illuminance: 30000.0,
			..default()
		},
		transform: Transform::from_xyz(40000.0, 2000.0, 40000.0)
			.with_rotation(Quat::from_rotation_x(-3.14 / 4.)),
		..default()
	});


	// Terrain grid
	commands.spawn_bundle(PbrBundle {
		transform: Transform::from_scale(Vec3::splat(75.0)),

		mesh: meshes.add(terrain::generate_grid(SIZE)),
		material: materials.add(StandardMaterial {
			base_color: Color::rgb(1.0, 1.0, 1.0),
			metallic: 0.0,
			..default()
		}),
		..Default::default()
	});

	// Orientation sphere
	commands.spawn_bundle(PbrBundle {
		transform: Transform::from_translation(Vec3::new(40000.0, 2000.0,40000.0)),
		mesh: meshes.add(Mesh::from(shape::Icosphere {
			radius: 2000.0,
			subdivisions: 3,
		})),

		material: materials.add(StandardMaterial {
			base_color: Color::rgb(1.0, 1.0, 1.0),
			..default()
		}),
		..Default::default()
	});

	
	let skybox_box = meshes.add(Mesh::from(shape::Icosphere {
		radius: 2000.0,
		subdivisions: 3,
	}));

	println!("Normals: {:?}", terrain::inv_norm(Mesh::from(shape::Icosphere {
		radius: 2000.0,
		subdivisions: 3,
	}).attribute(Mesh::ATTRIBUTE_NORMAL)));


	// this material renders the texture normally
	let material_handle = materials.add(StandardMaterial {
		base_color_texture: Some(texture_handle.clone()),
		alpha_mode: AlphaMode::Blend,
		unlit: true,
		..default()
	});

	// textured quad - normal
	commands.spawn_bundle(PbrBundle {
		mesh: skybox_box.clone(),
		// transform: Transform::from_scale(Vec3::splat(100.0)),
		material: material_handle,
		..default()
	});

}