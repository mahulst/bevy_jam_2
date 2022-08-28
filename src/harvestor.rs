use crate::field::{FIELD_MARGIN_SIZE, FIELD_SIZE, FIELD_THICKNESS};
use crate::ui::{
    update_help_text, ArrowImage, CommandsContainerMarker, CountDownMarkerMilliSeconds,
    CountDownMarkerSeconds, FontHandle, HelpTextContainer,
};
use bevy::prelude::*;
use bevy_easings::EaseFunction::QuadraticIn;
use bevy_easings::*;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use iyes_loopless::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::f32::consts::PI;
use std::time::Instant;

pub struct HarvestorPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HarvestorState {
    AcceptingCommands,
    Running,
    Done,
}

impl Plugin for HarvestorPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(HarvestorState::AcceptingCommands)
            .add_event::<HarvestorCommandsClearedEvent>()
            .add_system(move_harvestor)
            .init_resource::<TimeSpentWaitingOnCommands>()
            .add_enter_system(HarvestorState::AcceptingCommands, reset_time_waiting)
            .add_system(update_count_down.run_in_state(HarvestorState::AcceptingCommands))
            .register_inspectable::<Harvestor>()
            .register_inspectable::<InputCommands>()
            .add_system(watch_havestor_finished_moves.before(move_harvestor))
            .add_system(keyboard_input.before(move_harvestor))
            .add_plugin(EasingsPlugin)
            .add_enter_system(HarvestorState::AcceptingCommands, setup);
    }
}

pub struct TimeSpentWaitingOnCommands {
    time_start: Instant,
}

impl Default for TimeSpentWaitingOnCommands {
    fn default() -> Self {
        Self {
            time_start: Instant::now(),
        }
    }
}

fn reset_time_waiting(mut time_waiting: ResMut<TimeSpentWaitingOnCommands>) {
    time_waiting.time_start = Instant::now();
}

fn update_count_down(
    mut seconds_q: Query<
        &mut Text,
        (
            With<CountDownMarkerSeconds>,
            Without<CountDownMarkerMilliSeconds>,
        ),
    >,
    mut milliseconds_q: Query<
        &mut Text,
        (
            With<CountDownMarkerMilliSeconds>,
            Without<CountDownMarkerSeconds>,
        ),
    >,
    time_waiting: Res<TimeSpentWaitingOnCommands>,
) {
    let time_since = Instant::now() - time_waiting.time_start;
    let millis = time_since.as_millis().rem_euclid(1000) / 10;

    seconds_q.iter_mut().for_each(|mut text| {
        text.sections[0].value = format!("{}", time_since.as_secs());
    });
    milliseconds_q.iter_mut().for_each(|mut text| {
        text.sections[0].value = format!("{}", millis);
    });
}

pub struct HarvestorCommandsClearedEvent;

const HARVESTOR_SCALE: f32 = 0.0004;
const HARVESTOR_MOVEMENT_TIME: f32 = 0.25;

#[derive(Component, Inspectable, Default)]
pub struct Harvestor {
    pub position: IVec2,
    direction: HarvestorCommands,
    #[inspectable(ignore)]
    moving: Option<Timer>,
    turning: bool,
}

#[derive(Debug, Inspectable, Default, PartialEq, Eq, Clone)]
pub enum HarvestorCommands {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Distribution<HarvestorCommands> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HarvestorCommands {
        match rng.gen_range(0..=3) {
            0 => HarvestorCommands::Up,
            1 => HarvestorCommands::Down,
            2 => HarvestorCommands::Left,
            _ => HarvestorCommands::Right,
        }
    }
}

#[derive(Component, Inspectable, Default)]
struct InputCommands {
    commands: Vec<HarvestorCommands>,
    clear: bool,
}

fn setup(
    mut commands: Commands,
    ass: Res<AssetServer>,
    harvestor_q: Query<Entity, With<Harvestor>>,
) {
    harvestor_q.iter().for_each(|e| {
        commands.entity(e).despawn_recursive();
    });
    spawn(&mut commands, &ass, IVec2::new(0, -1));
}

fn spawn(commands: &mut Commands, ass: &Res<AssetServer>, position: IVec2) {
    let gltf: Handle<Scene> = ass.load("harvestor.glb#Scene0");

    let start_pos = Vec3::new(-1.0, FIELD_THICKNESS + 0.05, -0.2);
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
            position,
            direction: HarvestorCommands::Left,
            moving: None,
            turning: false,
        })
        .insert(InputCommands {
            commands: vec![],
            clear: false,
        });
}

pub fn command_to_direction(input: &HarvestorCommands) -> Vec3 {
    match input {
        HarvestorCommands::Up => Vec3::Z,
        HarvestorCommands::Down => -Vec3::Z,
        HarvestorCommands::Left => Vec3::X,
        HarvestorCommands::Right => -Vec3::X,
    }
}

pub fn watch_havestor_finished_moves(
    mut harvestor_q: Query<&mut Harvestor>,
    time: Res<Time>,
    mut ev_commands_cleared: EventWriter<HarvestorCommandsClearedEvent>,
    state: Res<CurrentState<HarvestorState>>,
) {
    harvestor_q.iter_mut().for_each(|mut h| {
        let delta = time.delta();
        if let Some(ref mut timer) = h.moving {
            timer.tick(delta);

            if timer.just_finished() {
                h.moving = None;
                if !h.turning {
                    let a = command_to_direction(&h.direction);

                    h.position.x -= a.x as i32;
                    h.position.y += a.z as i32;

                    if state.0 == HarvestorState::Done {
                        ev_commands_cleared.send(HarvestorCommandsClearedEvent);
                    }
                }
                h.turning = false;
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
                let vector_distance = dir * (FIELD_SIZE + FIELD_MARGIN_SIZE);
                if h.direction == *cmd {
                    let mut new_tf = *tf;
                    new_tf.translation += vector_distance;
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
                    let mut new_tf = *tf;
                    new_tf.look_at(vector_distance + tf.translation, Vec3::Y);
                    let a = tf.ease_to(
                        new_tf,
                        QuadraticIn,
                        EasingType::Once {
                            duration: std::time::Duration::from_secs_f32(HARVESTOR_MOVEMENT_TIME),
                        },
                    );
                    commands.entity(e).insert(a);
                    h.direction = cmd.clone();
                    h.turning = true;
                };
                h.moving = Timer::from_seconds(HARVESTOR_MOVEMENT_TIME, false).into();
            }

            if input_commands.commands.is_empty() {
                input_commands.clear = false;
                commands.insert_resource(NextState(HarvestorState::Done));
            }
        });
}

#[allow(clippy::too_many_arguments)]
fn keyboard_input(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut InputCommands>,
    ui: Query<Entity, With<CommandsContainerMarker>>,
    help_ui_container_q: Query<Entity, With<HelpTextContainer>>,
    arrow_image: Res<ArrowImage>,
    font: Res<FontHandle>,
    state: Res<CurrentState<HarvestorState>>,
) {
    let mut command = None;
    if state.0 == HarvestorState::AcceptingCommands {
        if keys.just_released(KeyCode::Left) {
            command = Some(HarvestorCommands::Left);
        } else if keys.just_released(KeyCode::Right) {
            command = Some(HarvestorCommands::Right);
        } else if keys.just_released(KeyCode::Up) {
            command = Some(HarvestorCommands::Up);
        } else if keys.just_released(KeyCode::Down) {
            command = Some(HarvestorCommands::Down);
        };
        if let Some(command) = command {
            // should update help text
            let mut update_help = false;
            query.iter().take(1).for_each(|h| {
                if h.commands.is_empty() {
                    update_help = true;
                }
            });

            if update_help {
                let e = help_ui_container_q.single();
                update_help_text(&font, &mut commands, e, "Press Enter to execute commands");
            }

            query.iter_mut().for_each(|mut ic| {
                ic.commands.push(command.clone());
            });

            let command_ui_entity = ui.single();
            spawn_image_command_ui(&arrow_image, &mut commands, command_ui_entity, &command);
        }

        if keys.just_released(KeyCode::Return) {
            let e = help_ui_container_q.single();
            update_help_text(&font, &mut commands, e, "Harvesting...");

            query.iter_mut().for_each(|mut ic| {
                commands.insert_resource(NextState(HarvestorState::Running));

                ic.clear = true;
            });
        }
    }
    if state.0 == HarvestorState::Done && keys.just_released(KeyCode::Space) {
        let command_ui_entity = ui.single();

        let mut command_ui_parent = commands.entity(command_ui_entity);
        command_ui_parent.despawn_descendants();
        commands.insert_resource(NextState(HarvestorState::AcceptingCommands));
    }
}

fn spawn_image_command_ui(
    arrow_image: &Res<ArrowImage>,
    commands: &mut Commands,
    ui_entity: Entity,
    command: &HarvestorCommands,
) {
    let mut command_ui_parent = commands.entity(ui_entity);
    command_ui_parent.with_children(|p| {
        let degrees = match command {
            HarvestorCommands::Up => 180.0,
            HarvestorCommands::Down => 0.0,
            HarvestorCommands::Left => 270.0,
            HarvestorCommands::Right => 90.0,
        };

        let radians = PI / 180.0 * degrees;
        p.spawn_bundle(ImageBundle {
            transform: Transform::default().with_rotation(Quat::from_axis_angle(Vec3::Z, radians)),
            style: Style {
                size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                margin: UiRect {
                    left: Val::Px(0.0),
                    right: Val::Px(8.0),
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                },
                ..default()
            },
            image: arrow_image.handle.clone().into(),
            ..default()
        });
    });
}
