#![allow(unused)]

use avian3d::{math::*, prelude::*};
use bevy::{app::App, prelude::*};

use crate::bevy_starter::camera::MainCamera;

#[derive(Clone, PartialEq, Eq, Hash, Debug, States, Default)]
enum CameraState {
    #[default]
    StaticView,
    FirstPersonView,
}

impl CameraState{
    fn toggle(&self) -> Self {
        match self {
            CameraState::StaticView => CameraState::FirstPersonView,
            CameraState::FirstPersonView => CameraState::StaticView,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, States, Default)]
enum AppState {
    #[default]
    InWorld,
    Menu,
}

pub(super) fn plugin(app: &mut App) {
    // Your game logic here
    app
    .add_systems(Startup, (make_main_character))
    .init_state::<CameraState>()
    .init_state::<AppState>()
    .add_systems(OnEnter(CameraState::StaticView), camera_static_view)
    .add_systems(Update, camera_first_person_view.run_if(in_state(CameraState::FirstPersonView)))
    .add_systems(Update, (set_camera_state));
    println!("games plugin")
}

#[derive(Component)]
struct MainCharacter;

fn make_main_character(mut commands: Commands) {
    commands.spawn((
        MainCharacter,
        Transform::default()
    ));
}

fn set_camera_state(mut next_state: ResMut<NextState<CameraState>>, current_state: Res<State<CameraState>>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_released(KeyCode::Backspace) {
        let camera_state = current_state.get();
        next_state.set(current_state.toggle());
    }
}

fn camera_first_person_view(mut main_camera_query: Query<&mut Transform, With<MainCamera>>, main_character_query: Query<&Transform,(With<MainCharacter>, Without<MainCamera>)>){
    if let Ok(mut main_camera_transform) = main_camera_query.single_mut() && 
    let Ok(main_character_transform) = main_character_query.single(){   
        main_camera_transform.translation = main_character_transform.translation;
    }
}

fn camera_static_view(mut main_camera_query: Query<&mut Transform, With<MainCamera>>) {

    if let Ok(mut main_camera_transform) = main_camera_query.single_mut() {
        *main_camera_transform = Transform::from_xyz(0.0, 2.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y);
    }
}