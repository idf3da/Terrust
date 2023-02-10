use std::string;

use bevy::{prelude::*, render::camera::Projection};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use crate::chunk::*;

#[derive(Default)]
pub struct OccupiedScreenSpace {
        pub left: f32,
}

const CAMERA_TARGET: Vec3 = Vec3::ZERO;

pub struct OriginalCameraTransform(Transform);

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
pub fn ui_example_system(
        mut egui_context: ResMut<EguiContext>,
        mut ui_state: ResMut<UiState>,
        mut rendered_texture_id: Local<egui::TextureId>,
        mut query: Query<Entity, With<MeshIndicator>>, 
        mut commands: Commands,
        mut chunks: ResMut<ChunksResource>,
        mut is_initialized: Local<bool>,
        // mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
        egui::SidePanel::left("left_panel").default_width(300.0).resizable(true).show(egui_context.ctx_mut(), |ui| {
                ui.heading("Terrust - a terrain generator");

                ui.horizontal(|ui| {
                        ui.label("Render distance in chunks");
                });

                
                ui.horizontal(|ui| {
                        ui.add(egui::Slider::new(&mut ui_state.render_distance, 1..=8));
                        if ui.button("-").clicked() {
                                println!("Chunk -1");
                                ui_state.render_distance += 1;
                        }
                        if ui.button("+").clicked() {
                                println!("Chunk +1");
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
                                println!("Save button.");
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

pub fn update_camera_transform_system (
        occupied_screen_space: Res<crate::ui::OccupiedScreenSpace>,
        original_camera_transform: Res<OriginalCameraTransform>,
        windows: Res<Windows>,
        mut camera_query: Query<(&Projection, &mut Transform)>,
) {
        let (camera_projection, mut transform) = match camera_query.get_single_mut() {
                Ok((Projection::Perspective(projection), transform)) => (projection, transform),
                _ => unreachable!(),
        };

        let distance_to_target = (CAMERA_TARGET - original_camera_transform.0.translation).length();
        let frustum_height = 2.0 * distance_to_target * (camera_projection.fov * 0.5).tan();
        let frustum_width = frustum_height * camera_projection.aspect_ratio;

        let window = windows.get_primary().unwrap();

        let left_taken = occupied_screen_space.left / window.width();
        transform.translation = original_camera_transform.0.translation
                + transform.rotation.mul_vec3(Vec3::new(
                (0.0 - left_taken) * frustum_width * 0.5,
                (0.0) * frustum_height * 0.5,
                0.0,
        ));
}