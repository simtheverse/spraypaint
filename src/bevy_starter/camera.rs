use bevy::prelude::*;

#[derive(Component)]
#[require(Camera3d)]
pub struct MainCamera;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, initialize_camera);
}

fn initialize_camera(_commands: Commands) {
    //commands.spawn(MainCamera);
}