use bevy::prelude::*;

use spraypaint::bevy_starter::AppPlugin as bevy_starter;
use spraypaint::simple_scene::SimpleScenePlugin as simple_scene;
use spraypaint::character_controller::CharacterControllerPlugin as character_controller;
use spraypaint::physics::ExampleCommonPlugin as physics_plugin;
use spraypaint::camera::CameraPlugin as camera_plugin;

fn main() {
    App::new()
    .add_plugins(bevy_starter)
    .add_plugins(simple_scene)
    .add_plugins(physics_plugin)
    .add_plugins(character_controller)
    .add_plugins(camera_plugin)
    .run();
}