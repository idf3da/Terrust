use bevy::prelude::*;
use crate::mathop::*;


pub fn move_camera(mut camera: Query<&mut Transform, With<Camera>>) {
        for mut cam in camera.iter_mut() {
                println!("{:?}", cam.translation);
        }
}
