use bevy::{input::mouse::MouseMotion, prelude::*};

use crate::simple_scene::game::{MainCamera, spawn_main_camera};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup.after(spawn_main_camera));
    app.add_systems(Update, set_look_at);
    app.init_resource::<MovementSettings>();
    app.add_systems(Update, cursor_grab);
}

/// Mouse sensitivity and movement speed
#[derive(Resource)]
pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00005,
            speed: 12.,
        }
    }
}

fn setup(_commands: Commands, player_entity_query: Query<Entity, With<MainCamera>>) {
    let _player_entity = player_entity_query.single().unwrap();
    //commands.entity(player_entity).insert(CameraSensitivity::default());
}

use bevy::window::{CursorGrabMode, PrimaryWindow};
/// Handles looking around if cursor is locked
fn set_look_at(
    settings: Res<MovementSettings>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut state: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<MainCamera>>,
) {
    if let Ok(window) = primary_window.single() {
        for mut transform in query.iter_mut() {
            for ev in state.read() {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor_options.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                        let window_scale = window.height().min(window.width());
                        pitch -= (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                        yaw -= (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);


                let rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, 0.0);
                transform.rotation = rotation;
            }
        }
    } else {
        warn!("Primary window not found for `player_look`!");
    }
}

fn cursor_grab(
    keys: Res<ButtonInput<KeyCode>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    if let Ok(mut window) = primary_window.single_mut() {
        if keys.just_pressed(KeyCode::Escape) {
            toggle_grab_cursor(&mut window);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

/// Grabs/ungrabs mouse cursor
fn toggle_grab_cursor(window: &mut Window) {
    match window.cursor_options.grab_mode {
        CursorGrabMode::None => {
            window.cursor_options.grab_mode = CursorGrabMode::Confined;
            window.cursor_options.visible = false;
        }
        _ => {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        }
    }
}