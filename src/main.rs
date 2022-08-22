mod wheat;
mod wheat_mesh;
mod harvestor;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use crate::harvestor::{ HarvestorPlugin};
use crate::shape::Cube;
use crate::wheat::WheatPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        // .add_plugin(WheatPlugin)
        .add_plugin(HarvestorPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(4.0001, 3.0, 0.0))
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

