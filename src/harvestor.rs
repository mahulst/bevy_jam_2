use crate::Keyframes::Rotation;
use bevy::prelude::*;
use bevy_easings::EaseFunction::QuadraticIn;
use bevy_easings::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

pub struct HarvestorPlugin;

impl Plugin for HarvestorPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(move_harvestor)
            .register_inspectable::<Harvestor>()
            .register_inspectable::<InputCommands>()
            .add_system(watch_havestor_finished_moves.before(move_harvestor))
            .add_system(keyboard_input.before(move_harvestor))
            .add_plugin(EasingsPlugin);
    }
}
const HARVESTOR_SCALE: f32 = 0.001;
const HARVESTOR_MOVEMENT_TIME: f32 = 5.0;

#[derive(Debug, Inspectable, Default, PartialEq, Clone)]
enum HarvestorCommands {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Inspectable, Default)]
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
            ])).looking_at(command_to_direction(&HarvestorCommands::Left), Vec3::Y),

            ..Default::default()
        })
        .insert(Harvestor {
            position: position,
            direction: HarvestorCommands::Left,
            moving: None,
        })
        .insert(InputCommands {
            commands: vec![],
            clear: false,
        });
}

#[derive(Component, Inspectable, Default)]
struct Harvestor {
    position: Vec2,
    direction: HarvestorCommands,
    #[inspectable(ignore)]
    moving: Option<Timer>,
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
        HarvestorCommands::Right => 0.0,
        HarvestorCommands::Down => 90.0,
        HarvestorCommands::Left => 180.0,
        HarvestorCommands::Up => 270.0,
    }
}

fn watch_havestor_finished_moves(
    mut harvestor_q: Query<(Entity, &Transform, &mut Harvestor)>,
    time: Res<Time>,
) {
    harvestor_q.iter_mut().for_each(|(e, tf, mut h)| {
        let delta = time.delta();
        if let Some(ref mut timer) = h.moving {
            timer.tick(delta);

            if timer.just_finished() {
                h.moving = None;
            }
        }
    });
}

fn move_harvestor(
    mut commands: Commands,
    mut harvestor_q: Query<(Entity, &Transform, &mut InputCommands, &mut Harvestor)>,
) {
    harvestor_q
        .iter_mut()
        .for_each(|(e, tf, mut input_commands, mut h)| {
            if !input_commands.clear || input_commands.commands.is_empty() {
                return;
            }

            if h.moving.is_some() {
                return;
            }
            if let Some(cmd) = input_commands.commands.get(0) {
                println!("receiving command {:?} (now pointing {:?})", cmd, h.direction);
                let dir = command_to_direction(cmd);
                if h.direction == *cmd {
                    let mut new_tf = tf.clone();
                    new_tf.translation += dir;
                    let a = tf.ease_to(
                        new_tf,
                        QuadraticIn,
                        bevy_easings::EasingType::Once {
                            duration: std::time::Duration::from_secs_f32(HARVESTOR_MOVEMENT_TIME),
                        },
                    );
                    println!("Only move {:?} (current pos {:?} to {:?})", cmd, tf.translation, new_tf.translation);
                    dbg!(&a);
                    input_commands.commands.remove(0);
                    commands.entity(e).insert(a);
                } else {
                    println!("Turn {:?} (from h.direction {:?})", cmd, h.direction);
                    let mut new_tf = tf.clone();
                    new_tf.look_at(dir + tf.translation, Vec3::Y);
                    let a = tf.ease_to(
                        new_tf,
                        QuadraticIn,
                        EasingType::Once {
                            duration: std::time::Duration::from_secs_f32(HARVESTOR_MOVEMENT_TIME),
                        },
                    );
                    commands.entity(e).insert(a);
                    h.direction = cmd.clone();
                };
                h.moving = Timer::from_seconds(HARVESTOR_MOVEMENT_TIME, false).into();
            }

            if input_commands.commands.is_empty() {
                input_commands.clear = false;
            }
        });
}

fn keyboard_input(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &Harvestor, &mut InputCommands)>,
) {
    if keys.just_released(KeyCode::Left) {
        query.iter_mut().for_each(|(e, h, mut ic)| {
            ic.commands.push(HarvestorCommands::Left);
        });
    } else if keys.just_released(KeyCode::Right) {
        query.iter_mut().for_each(|(e, h, mut ic)| {
            ic.commands.push(HarvestorCommands::Right);
        });
    } else if keys.just_released(KeyCode::Up) {
        query.iter_mut().for_each(|(e, h, mut ic)| {
            ic.commands.push(HarvestorCommands::Up);
        });
    } else if keys.just_released(KeyCode::Down) {
        query.iter_mut().for_each(|(e, h, mut ic)| {
            ic.commands.push(HarvestorCommands::Down);
        });
    };
    if keys.just_released(KeyCode::Return) {
        query.iter_mut().for_each(|(e, h, mut ic)| {
            ic.clear = true;
        });
    }
}
