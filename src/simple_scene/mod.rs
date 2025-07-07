use bevy::prelude::*;

pub mod simple_scene;
pub mod game;

pub fn add_all_plugins(app: &mut App) {
    app.add_plugins(simple_scene::plugin);
    app.add_plugins(game::plugin);
}

pub struct SimpleScenePlugin;
impl Plugin for SimpleScenePlugin {
    fn build(&self, app: &mut App) {
        add_all_plugins(app);
    }
}
