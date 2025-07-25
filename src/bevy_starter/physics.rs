use avian3d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    // Add physics plugins and specify a units-per-meter scaling factor, 1 meter = 20 pixels. The
    // unit allows the engine to tune its parameters for the scale of the world, improving
    // stability.
    app.add_plugins(PhysicsPlugins::default());
}