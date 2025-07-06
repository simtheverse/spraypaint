use bevy::prelude::{App, Commands, Startup};
use iyes_perf_ui::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app
    .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
    .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin::default())
    .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin::default())
    .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin::default())
    .add_plugins(PerfUiPlugin)
    .add_systems(Startup, setup);
}

fn setup(mut commands: Commands) {
    // create a simple Perf UI with default settings
    // and all entries provided by the crate:
    commands.spawn(PerfUiAllEntries::default());
}