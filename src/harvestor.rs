use crate::field::{FIELD_MARGIN_SIZE, FIELD_SIZE, FIELD_THICKNESS};
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
const HARVESTOR_SCALE: f32 = 0.0004;
const HARVESTOR_MOVEMENT_TIME: f32 = 0.5;

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

    let start_pos = Vec3::new(-0.9, FIELD_THICKNESS + 0.05, -0.2);
    commands
        .spawn_bundle(SceneBundle {
            scene: gltf,
            transform: Transform::from_xyz(start_pos.x, start_pos.y, start_pos.z)
                .with_scale(Vec3::from([
                    HARVESTOR_SCALE,
                    HARVESTOR_SCALE,
                    HARVESTOR_SCALE,
                ]))
                .looking_at(
                    command_to_direction(&HarvestorCommands::Left) + start_pos,
                    Vec3::Y,
                ),

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
    let vec = match input {
        HarvestorCommands::Up => Vec3::Z,
        HarvestorCommands::Down => -Vec3::Z,
        HarvestorCommands::Left => Vec3::X,
        HarvestorCommands::Right => -Vec3::X,
    };
    vec * (FIELD_SIZE + FIELD_MARGIN_SIZE)
}

fn watch_havestor_finished_moves(mut harvestor_q: Query<&mut Harvestor>, time: Res<Time>) {
    harvestor_q.iter_mut().for_each(|mut h| {
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
    mut harvestor_q: Query<
        (Entity, &Transform, &mut InputCommands, &mut Harvestor),
        Without<EasingComponent<Transform>>,
    >,
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
                let dir = command_to_direction(cmd);
                if h.direction == *cmd {
                    let mut new_tf = tf.clone();
                    new_tf.translation += dir;
                    let easing_component = tf.ease_to(
                        new_tf,
                        QuadraticIn,
                        bevy_easings::EasingType::Once {
                            duration: std::time::Duration::from_secs_f32(HARVESTOR_MOVEMENT_TIME),
                        },
                    );
                    commands.entity(e).insert(easing_component);
                    input_commands.commands.remove(0);
                } else {
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

fn keyboard_input(keys: Res<Input<KeyCode>>, mut query: Query<&mut InputCommands>) {
    if keys.just_released(KeyCode::Left) {
        query.iter_mut().for_each(|mut ic| {
            ic.commands.push(HarvestorCommands::Left);
        });
    } else if keys.just_released(KeyCode::Right) {
        query.iter_mut().for_each(|mut ic| {
            ic.commands.push(HarvestorCommands::Right);
        });
    } else if keys.just_released(KeyCode::Up) {
        query.iter_mut().for_each(|mut ic| {
            ic.commands.push(HarvestorCommands::Up);
        });
    } else if keys.just_released(KeyCode::Down) {
        query.iter_mut().for_each(|mut ic| {
            ic.commands.push(HarvestorCommands::Down);
        });
    };
    if keys.just_released(KeyCode::Return) {
        query.iter_mut().for_each(|mut ic| {
            ic.clear = true;
        });
    }
}
