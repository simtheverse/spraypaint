use avian3d::prelude::{Collider, ColliderConstructor, RigidBody, TransformInterpolation};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

#[derive(Component)]
pub struct Block;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(20.0, 0.5))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        ColliderConstructor::Cylinder { radius: (20.0), height: (0.5) },
        RigidBody::Static,
    ));

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Block,
        Collider::cuboid(1.0, 1.0, 1.0),
        RigidBody::Dynamic,
        TransformInterpolation
    ));
    // light
    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            range: 50.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}