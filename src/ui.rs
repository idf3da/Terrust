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
        egui_texture_handle: Option<egui::TextureHandle>,
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

                
                ui.add(egui::Slider::new(&mut ui_state.render_distance, 1..=8));
                ui.horizontal(|ui| {
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

                if ui.button("Flush chunks").clicked() {
                        println!("Flush button.");
                        chunks.flush(query, commands);
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