#![allow(unused)]

use avian3d::{math::*, prelude::*};
use bevy::{app::App, prelude::*};

use crate::character_controller::CharacterControllerBundle;

const INITIAL_HEIGHT: f32 = 3.0;

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
    .add_systems(Startup, make_main_camera)
    .init_state::<CameraState>()
    .init_state::<AppState>()
    .add_systems(OnEnter(CameraState::StaticView), camera_static_view)
    .add_systems(OnEnter(CameraState::FirstPersonView), player_translation_reset)
    .add_systems(Update, camera_first_person_view.run_if(in_state(CameraState::FirstPersonView)))
    .add_systems(Update, (set_camera_state));
    println!("games plugin")
}

#[derive(Component)]
struct MainCharacter;

#[derive(Component)]
#[require(Camera3d)]
pub struct MainCamera;

fn make_main_character(mut commands: Commands) {
    commands.spawn((
        MainCharacter,
        Transform::from_xyz(0.0, INITIAL_HEIGHT, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        CharacterControllerBundle::new(Collider::capsule(0.4, 1.0), Vector::NEG_Y * 9.81 *2.0 ).with_movement(
            90.0,
            0.985,
            7.5,
            (30.0 as Scalar).to_radians(),
        ),
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        TransformInterpolation,
        //GravityScale(0.0),
        )
    );
}

fn make_main_camera(mut commands: Commands) {
        commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, INITIAL_HEIGHT, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,));
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
        *main_camera_transform = Transform::from_xyz(0.0, INITIAL_HEIGHT, 8.0).looking_at(Vec3::ZERO, Vec3::Y);
    }
}


fn player_translation_reset(main_camera_query: Query<&Transform, With<MainCamera>>, mut main_character_query: Query<&mut Transform,(With<MainCharacter>, Without<MainCamera>)>) {
    if let Ok(main_camera_transform) = main_camera_query.single() && 
    let Ok(mut main_character_transform) = main_character_query.single_mut(){   
        main_character_transform.translation = main_camera_transform.translation;
    }
}