use bevy::prelude::*;

pub mod fps_controller;

pub fn add_all_plugins(app: &mut App) {
    app.add_plugins(fps_controller::plugin);
}

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        add_all_plugins(app);
    }
}
