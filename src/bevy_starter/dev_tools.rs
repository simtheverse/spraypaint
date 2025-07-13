//! Development tools for the game. This plugin is only enabled in dev builds.
#![cfg(feature = "dev")]

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub(crate) fn plugin(app: &mut App) {
    let toggle_system = toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY));

    // Toggle the debug overlay for UI.
    app.add_systems(Update, toggle_system);
}

const TOGGLE_KEY: KeyCode = KeyCode::Numpad1;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}