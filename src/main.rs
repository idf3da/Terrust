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


const CLEAR: Color = Color::rgb(0.1, 0.1, 0.1);
const SUN: Color = Color::rgb(0.992156, 0.721568, 0.074509);
const S: u32 = 0;
const SIZE: (u32, u32) = (2 << S, 2 << S);
const HALF_SIZE: f32 = 1.0;

struct CubemapMaterial {
	base_color_texture: Option<Handle<Image>>,
}

impl Material for CubemapMaterial {
	fn fragment_shader() -> ShaderRef {
		"shaders/cubemap_unlit.wgsl".into()
	}

	fn specialize(
		_pipeline: &MaterialPipeline<Self>,
		descriptor: &mut RenderPipelineDescriptor,
		_layout: &MeshVertexBufferLayout,
_key: MaterialPipelineKey<Self>,
	) -> Result<(), SpecializedMeshPipelineError> {
		descriptor.primitive.cull_mode = None;
		Ok(())
	}
}

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
			features: WgpuFeatures::POLYGON_MODE_LINE,
			..default()
		})
		.add_plugins(DefaultPlugins)
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
	mut materials: ResMut<Assets<StandardMaterial>>,
) {
	wireframe_config.global = true;

	commands.spawn_bundle(DirectionalLightBundle {
		directional_light: DirectionalLight {
			illuminance: 30000.0,
			..default()
		},
		transform: Transform::from_xyz(40000.0, 2000.0, 40000.0)
			.with_rotation(Quat::from_rotation_x(-3.14 / 4.)),
		..default()
	});


	commands.spawn_bundle(PbrBundle {
		transform: Transform::from_scale(Vec3::splat(75.0)),

		mesh: meshes.add(terrain::generate_grid(SIZE)),
		material: materials.add(StandardMaterial {
			base_color: Color::rgb(1.0, 1.0, 1.0),
			..default()
		}),
		..Default::default()
	});

	commands.spawn_bundle(PbrBundle {
		transform: Transform::from_translation(Vec3::new(40000.0, 2000.0,40000.0)),
		mesh: meshes.add(Mesh::from(shape::Icosphere {
			radius: 200.0,
			subdivisions: 3,
		})),
		material: materials.add(StandardMaterial {
			base_color: Color::rgb(1.0, 1.0, 1.0),
			..default()
		}),
		
	        ..Default::default()
	});

	commands.spawn(MaterialMeshBundle::<CubemapMaterial> {
        	mesh: meshes.add(Mesh::from(shape::Cube { size: 10000.0 })),
                material: cubemap_materials.add(CubemapMaterial {
                	base_color_texture: Some(cubemap.image_handle.clone_weak()),
                }),
                ..default()
            });
	
}