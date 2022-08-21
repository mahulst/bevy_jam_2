mod wheat;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use crate::shape::Cube;
use crate::wheat::WheatPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_plugin(WheatPlugin)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(spawn_gltf)
        .add_plugin(WorldInspectorPlugin::new())
        .run();
}
const HARVESTOR_SCALE: f32 = 0.001;
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

struct MyAssetPack(Handle<Gltf>);

fn spawn_gltf(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let gltf: Handle<Scene> = ass.load("harvestor.glb#Scene0");


    commands
        .spawn_bundle(SceneBundle {
            scene: gltf,
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::from([
                HARVESTOR_SCALE,
                HARVESTOR_SCALE,
                HARVESTOR_SCALE,
            ])),

            ..Default::default()
        });
}
