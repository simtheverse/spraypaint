use bevy::prelude::*;

use spraypaint::bevy_starter::AppPlugin as bevy_starter;
use spraypaint::simple_scene::SimpleScenePlugin as simple_scene;

fn main() {
    App::new()
    .add_plugins(bevy_starter)
    .add_plugins(simple_scene)
    .run();
}