use bevy::prelude::*;

mod bevy_starter;
pub use bevy_starter::prelude::*;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        bevy_starter::add_all_plugins(app);
    }
}