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
    let ground=Vec2::new(20.0, 0.5);
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(ground.x, ground.y))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        ColliderConstructor::Cylinder { radius: (ground.x), height: (ground.y) },
        RigidBody::Static,
    ));

    // Wall
    // Define the cuboid dimensions
    let cuboid_size = Vec3::new(10.0, 2.0, 0.2);    
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(cuboid_size.x, cuboid_size.y, cuboid_size.z))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, ground.y/2.0+cuboid_size.y/2.0, 0.0),
        Block,
        Collider::cuboid(cuboid_size.x, cuboid_size.y, cuboid_size.z),
        RigidBody::Static,
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