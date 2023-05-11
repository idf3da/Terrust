use std::string;

use bevy::{prelude::*, render::camera::Projection};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use crate::chunk::*;

// Shared UiState, stores all UI values
#[derive(Default)]
pub struct UiState {
        file_name: String,
        pub render_distance: u32,
        pub int_location: (u32, u32),
        pub layers: Vec<[i32; 3]>,
        l1s: i32,
        l1h: i32,
        l1e: bool,
        l2s: i32,
        l2h: i32,
        l2e: bool,
        l3s: i32,
        l3h: i32,
        l3e: bool,
        l4s: i32,
        l4h: i32,
        l4e: bool,
        egui_texture_handle: Option<egui::TextureHandle>,
}


// Pre-set defaults for UiState
pub fn ui_state_defaults(
        mut ui_state: ResMut<UiState>
) {
        ui_state.render_distance = 3;
        
        ui_state.l1s = 7;
        ui_state.l1h = 40;
        ui_state.l1e = true;

        ui_state.l2s = 14;
        ui_state.l2h = 20;
        ui_state.l2e = true;

        ui_state.l3s = 21;
        ui_state.l3h = 10;
        ui_state.l3e = true;

        ui_state.l4s = 28;
        ui_state.l4h = 5;
        ui_state.l4e = true;
}
// TODO: When custom camera controls enable perspective shift
pub fn ui_system(
        mut egui_context: ResMut<EguiContext>,
        mut ui_state: ResMut<UiState>,
        mut query: Query<Entity, With<MeshIndicator>>, 
        mut commands: Commands,
        mut chunks: ResMut<ChunksResource>,
) {
        egui::SidePanel::left("left_panel").default_width(300.0).resizable(true).show(egui_context.ctx_mut(), |ui| {
                ui.heading("Terrust - a terrain generator");

                ui.horizontal(|ui| {
                        ui.label("Render distance in chunks");
                });

                
                ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(&mut ui_state.render_distance, 1..=8));
                        if ui.button("-").clicked() {
                                ui_state.render_distance += 1;
                        }
                        if ui.button("+").clicked() {
                                ui_state.render_distance += 1;
                        }
                });
                ui.allocate_space(egui::Vec2::new(0.0, 50.0));

                ui.horizontal(|ui| {
                        ui.label("Perlin layers (frequency | height | enable)");
                });

                ui.horizontal(|ui| {
                        ui.label("#1");
                        ui.add(egui::Slider::new(&mut ui_state.l1s, 0..=64));
                        ui.add(egui::Slider::new(&mut ui_state.l1h, 0..=64));
                        ui.checkbox(&mut ui_state.l1e, "");
                });

                ui.horizontal(|ui| {
                        ui.label("#2");
                        ui.add(egui::Slider::new(&mut ui_state.l2s, 0..=64));
                        ui.add(egui::Slider::new(&mut ui_state.l2h, 0..=64));
                        ui.checkbox(&mut ui_state.l2e, "");
                });

                ui.horizontal(|ui| {
                        ui.label("#3");
                        ui.add(egui::Slider::new(&mut ui_state.l3s, 0..=64));
                        ui.add(egui::Slider::new(&mut ui_state.l3h, 0..=64));
                        ui.checkbox(&mut ui_state.l3e, "");
                });

                ui.horizontal(|ui| {
                        ui.label("#4");
                        ui.add(egui::Slider::new(&mut ui_state.l4s, 0..=64));
                        ui.add(egui::Slider::new(&mut ui_state.l4h, 0..=64));
                        ui.checkbox(&mut ui_state.l4e, "");
                });
                ui.allocate_space(egui::Vec2::new(0.0, 50.0));

                // Reloads chunks by flushing all of them, chunk_update_system immidiatelly regenerates them
                if ui.button("Reload chunks").clicked() {
                        println!("Flush button.");
                        chunks.flush(query, commands);
                        ui_state.layers = vec![
                                [ui_state.l1s, ui_state.l1h, if ui_state.l1e {1} else {0}],
                                [ui_state.l2s, ui_state.l2h, if ui_state.l2e {1} else {0}],
                                [ui_state.l3s, ui_state.l3h, if ui_state.l3e {1} else {0}],
                                [ui_state.l4s, ui_state.l4h, if ui_state.l4e {1} else {0}]
                        ]
                }

                ui.label(format!("({}, {})", ui_state.int_location.0.to_string(), ui_state.int_location.1.to_string()));

                ui.allocate_space(egui::Vec2::new(0.0, 50.0));
                
                ui.horizontal(|ui| {
                        ui.label("File Name: ");
                        ui.text_edit_singleline(&mut ui_state.file_name);
                });

                ui.horizontal(|ui| {
                        if ui.button("Save to OBJ file").clicked() {
                                println!("Save button pressed.");
                                chunks.save(ui_state.int_location, ui_state.file_name.clone())
                        };
                });

                ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                        ui.add(egui::Hyperlink::from_label_and_url(
                                "This project is available on github",
                                "https://github.com/idf3da/terrust/",
                        ));
                });

                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        });
}