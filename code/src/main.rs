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

use bevy_egui::egui;
use bevy::prelude::*;
use bevy_flycam::{PlayerPlugin, MovementSettings};
use bevy_egui::{EguiPlugin};

const S: u32 = 8;
const CHUNK_RES: (u32, u32) = (2 << S, 2 << S);
const CHUNK_MAX_NUM: (u32, u32) = (64, 64);
const SCALE: f32 = (S * S * 100) as f32;
const PERLIN_HEIGHT_SCALE: f64 = 2.0;
const PERLIN_FREQ: f64 = 0.0009765625;

fn main() {
	println!("Starting Terrust!");
	App::new()
		//Initialise bevy app
		.add_plugins(DefaultPlugins)
		.insert_resource(WindowDescriptor {
			height: 800.0,
			width: 600.0,
			title: "Terrust 1.4`".to_string(),
			..default()
		})
		
		//Setup the UI system
		.add_plugin(EguiPlugin)
		.add_system(ui_system)
		.init_resource::<UiState>()
		.add_startup_system(ui_state_defaults)
		
		.init_resource::<ChunksResource>()
		
		//Add movement camera
		.add_plugin(PlayerPlugin)
		.insert_resource(MovementSettings {
			sensitivity: 0.0002,
			speed: 1400.0,
		})
		
		//Create a world
		.add_startup_system(basic_scene)
		.add_startup_system(chunks_init)
	
		//Systems
		.add_system(chunks_update_sys)
		.add_system(chunk_timed_flush)
		.run();
}