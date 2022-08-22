use crate::Keyframes::Rotation;
use bevy::prelude::*;
use bevy_easings::EaseFunction::QuadraticIn;
use bevy_easings::*;

pub struct HarvestorPlugin;

impl Plugin for HarvestorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .insert_resource(InputCommands {
                commands: vec![],
                clear: false,
            })
            .add_system(move_harvestor)
            .add_system(keyboard_input)
            .add_plugin(EasingsPlugin);
    }
}
const HARVESTOR_SCALE: f32 = 0.001;

#[derive(Debug, PartialEq)]
enum HarvestorCommands {
    Up,
    Down,
    Left,
    Right,
}

struct InputCommands {
    commands: Vec<HarvestorCommands>,
    clear: bool,
}

fn setup(mut commands: Commands, ass: Res<AssetServer>) {
    spawn(&mut commands, &ass, Vec2::new(0.0, 1.0));
}

fn spawn(commands: &mut Commands, ass: &Res<AssetServer>, position: Vec2) {
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
        })
        .insert(Harvestor {
            position: position,
            direction: HarvestorCommands::Left,
        });
}

#[derive(Component)]
struct Harvestor {
    position: Vec2,
    direction: HarvestorCommands,
}

fn command_to_direction(input: &HarvestorCommands) -> Vec3 {
    match input {
        HarvestorCommands::Up => Vec3::Z,
        HarvestorCommands::Down => -Vec3::Z,
        HarvestorCommands::Left => Vec3::X,
        HarvestorCommands::Right => -Vec3::X,
    }
}
fn command_to_degrees(input: &HarvestorCommands) -> f32 {
    match input {
        HarvestorCommands::Up => 90.0,
        HarvestorCommands::Down => 270.0,
        HarvestorCommands::Left => 0.0,
        HarvestorCommands::Right => 180.0,
    }
}

fn move_harvestor(
    mut commands: Commands,
    harvestor_q: Query<(Entity, &Transform, &Harvestor), Without<EasingComponent<Transform>>>,
    mut input_commands: ResMut<InputCommands>,
) {
    if !input_commands.clear || input_commands.commands.is_empty() {
        return;
    }

    harvestor_q.iter().for_each(|(e, tf, h)| {
        let cmd = input_commands.commands.remove(0);
        dbg!(&cmd);
        let dir = command_to_direction(&cmd);
        let turn_first = if h.direction == cmd {
            let mut new_tf = tf.clone();
            new_tf.look_at(dir, Vec3::Y);
            tf.ease_to(
                new_tf,
                QuadraticIn,
                EasingType::Once {
                    duration: std::time::Duration::from_secs(0),
                },
            )
        } else {
            let mut new_tf = tf.clone();
            new_tf.look_at(dir, Vec3::Y);
            tf.ease_to(
                new_tf,
                QuadraticIn,
                EasingType::Once {
                    duration: std::time::Duration::from_secs(1),
                },
            )
        };
        let mut new_tf = tf.clone();
        new_tf.look_at(dir, Vec3::Y);
        new_tf.translation = Vec3::new(dir.x, dir.y, dir.z);
        let a = turn_first.ease_to(
            new_tf,
            QuadraticIn,
            bevy_easings::EasingType::Once {
                duration: std::time::Duration::from_secs(1),
            },
        );
        commands.entity(e).insert(a);
    });
    if input_commands.commands.is_empty() {
        input_commands.clear = false;
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut input_commands: ResMut<InputCommands>) {
    if keys.just_released(KeyCode::Left) {
        input_commands.commands.push(HarvestorCommands::Left);
    }
    if keys.just_released(KeyCode::Right) {
        input_commands.commands.push(HarvestorCommands::Right);
    }
    if keys.just_released(KeyCode::Up) {
        input_commands.commands.push(HarvestorCommands::Up);
    }
    if keys.just_released(KeyCode::Down) {
        input_commands.commands.push(HarvestorCommands::Down);
    }
    if keys.just_released(KeyCode::Return) {
        input_commands.clear = true;
    }
}
