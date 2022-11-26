use bevy::prelude::*;
use crate::mathop::*;
use crate::chunk::*;


pub fn move_camera(mut camera: Query<&mut Transform, With<Camera>>) {
        for mut cam in camera.iter_mut() {
                println!("{:?}", cam.translation);
        }
}

pub fn chunk_timed_flush(
        time: Res<Time>,
        mut query: Query<Entity, With<MeshIndicator>>, 
        mut commands: Commands,
        mut chunks: ResMut<ChunksResource>,
) {
        if time.seconds_since_startup() as i32 % 60 == 0 {
                chunks.flush(query, commands)
        }
}