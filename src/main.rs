use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use crate::field::FieldPlugin;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;

use crate::harvestor::HarvestorPlugin;
use crate::ui::UIPlugin;
use crate::wheat::WheatPlugin;

mod field;
mod harvestor;
mod ui;
mod wheat;
mod wheat_mesh;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_startup_system(setup)
        .add_plugin(UIPlugin)
        .add_system(bevy::window::close_on_esc)
        .add_plugin(WheatPlugin)
        .add_plugin(HarvestorPlugin)
        .add_plugin(FieldPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .run();
}
fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0.0001, 3.0, -4.0))
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });

    commands
        // light
        .spawn_bundle(DirectionalLightBundle {
            transform: Transform::from_xyz(-1.0, 2.0, 0.0)
                .looking_at(Vec3::new(1.0, 0.0, 1.0), Vec3::Y),
            ..default()
        });
}
