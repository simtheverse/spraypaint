use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::Window;
use bevy::input::keyboard::KeyCode;

#[derive(Component)]
#[require(Camera3d)]
pub struct MainCamera;

#[derive(Component)]
pub struct FlyCameraController {
    pub speed: f32,
    pub sensitivity: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl Default for FlyCameraController {
    fn default() -> Self {
        Self {
            speed: 5.0,
            sensitivity: 0.1,
            pitch: 0.0,
            yaw: 0.0,
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, initialize_camera)
        .add_systems(Update, (fly_camera_movement, fly_camera_look));
}

fn initialize_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 2.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,
        FlyCameraController::default(),
    ));
}

fn fly_camera_movement(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&FlyCameraController, &mut Transform)>,
) {
    for (controller, mut transform) in &mut query {
        let mut direction = Vec3::ZERO;
        let forward: Vec3 = transform.forward().into();
        let right: Vec3 = transform.right().into();
        if keyboard.pressed(KeyCode::KeyW) {
            direction += forward;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction -= forward;
        }
        if keyboard.pressed(KeyCode::KeyA) {
            direction -= right;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction += right;
        }
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * controller.speed * time.delta().as_secs_f32();
        }
    }
}

fn fly_camera_look(
    windows: Query<&Window>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&mut FlyCameraController, &mut Transform)>,
) {
    if !mouse_buttons.pressed(MouseButton::Right) {
        return;
    }
    let mut delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        delta += event.delta;
    }
    if delta == Vec2::ZERO {
        return;
    }
    for (mut controller, mut transform) in &mut query {
        controller.yaw -= delta.x * controller.sensitivity * 0.01;
        controller.pitch -= delta.y * controller.sensitivity * 0.01;
        controller.pitch = controller.pitch.clamp(-1.54, 1.54); // ~89 degrees
        let yaw_rot = Quat::from_rotation_y(controller.yaw);
        let pitch_rot = Quat::from_rotation_x(controller.pitch);
        transform.rotation = yaw_rot * pitch_rot;
    }
}