pub mod asset_tracking;
pub mod camera;
pub mod debug;
pub mod default;
pub mod dev_tools;
pub mod fonts;
pub mod game;
pub mod input;
pub mod physics;
pub mod utils;

pub fn add_all_plugins(app: &mut bevy::prelude::App) {
    app.add_plugins(asset_tracking::plugin);
    app.add_plugins(default::plugin);
    app.add_plugins(fonts::plugin);
    app.add_plugins(camera::plugin);
    app.add_plugins(physics::plugin);
    app.add_plugins(input::plugin);
    app.add_plugins(game::plugin);
    #[cfg(feature = "dev")]
    app.add_plugins((dev_tools::plugin, debug::plugin));
}

pub mod prelude {
    pub use super::utils::*;
}
