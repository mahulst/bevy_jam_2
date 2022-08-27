use crate::harvestor::{watch_havestor_finished_moves, Harvestor, HarvestorCommandsClearedEvent};
use crate::ui::{update_help_text, FontHandle, HelpTextContainer};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_fields)
            .add_system(mow_target_field.after(watch_havestor_finished_moves))
            .init_resource::<FieldMaterialResource>()
            .add_system(change_mowed_material)
            .add_system(compare_fields_on_commands_cleared.after(mow_target_field))
            .add_startup_system(setup)
            .register_inspectable::<Field>();
    }
}

#[derive(Default)]
struct FieldMaterialResource {
    mowed: Handle<StandardMaterial>,
    not_mowed: Handle<StandardMaterial>,
}
#[derive(Inspectable, PartialEq, Default, Debug)]
enum FieldType {
    #[default]
    Target,
    Canvas,
}

#[derive(Component, Inspectable, Default, PartialEq, Debug)]
pub struct Field {
    size: UVec2,
    field_type: FieldType,
    #[inspectable(ignore)]
    mowed: HashMap<(i32, i32), bool>,
}

fn setup(
    mut commands: Commands,
    mut field_material: ResMut<FieldMaterialResource>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let material_field_fresh = materials.add(FIELD_FRESH_COLOR.into());
    let material_field_mowed = materials.add(FIELD_MOWED_COLOR.into());
    field_material.not_mowed = material_field_fresh;
    field_material.mowed = material_field_mowed;

    let mut target_mowed = HashMap::new();
    target_mowed.insert((0, 0), true);
    commands.spawn().insert(Field {
        size: UVec2::new(10, 10),
        field_type: FieldType::Target,
        mowed: target_mowed,
    });
    commands.spawn().insert(Field {
        size: UVec2::new(10, 10),
        field_type: FieldType::Canvas,
        mowed: HashMap::new(),
    });
}

const FIELD_FRESH_COLOR: Color = Color::rgb(0.536, 0.389, 0.076);
const FIELD_MOWED_COLOR: Color = Color::rgb(0.4, 0.2, 0.0);

pub const FIELD_SIZE: f32 = 0.2;
pub const FIELD_MARGIN_SIZE: f32 = 0.01;
pub const FIELD_THICKNESS: f32 = 0.02;

#[derive(Component)]
pub struct FieldSquareMarker(UVec2);

fn render_fields(
    query: Query<(Entity, &Field), Added<Field>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    field_material: Res<FieldMaterialResource>,
) {
    let mesh = Mesh::from(shape::Cube { size: FIELD_SIZE });
    let handle = meshes.add(mesh);

    query.iter().for_each(|(e, field)| {
        let pos = match field.field_type {
            FieldType::Target => Vec3::X,
            FieldType::Canvas => -Vec3::X,
        };
        let field_type_offset = pos * 2.0 + Vec3::X;

        let mut entity = commands.entity(e);
        entity.insert_bundle(SpatialBundle { ..default() });

        (0..field.size.y)
            .flat_map(|x| (0..field.size.x).map(move |y| (x, y)))
            .for_each(|(x, y)| {
                let material = if *field.mowed.get(&(x as i32, y as i32)).unwrap_or(&false) {
                    field_material.mowed.clone()
                } else {
                    field_material.not_mowed.clone()
                };

                let world_x = -1.0 * x as f32 * (FIELD_SIZE + FIELD_MARGIN_SIZE);
                let world_y = y as f32 * (FIELD_SIZE + FIELD_MARGIN_SIZE);
                entity.with_children(|cb| {
                    cb.spawn()
                        .insert_bundle(PbrBundle {
                            mesh: handle.clone(),
                            material,
                            transform: Transform::from_xyz(
                                world_x + field_type_offset.x,
                                0.0,
                                world_y,
                            )
                            .with_scale(Vec3::new(
                                1.0,
                                FIELD_THICKNESS,
                                1.0,
                            )),
                            ..default()
                        })
                        .insert(FieldSquareMarker(UVec2::new(x, y)));
                });
            });
    });
}

// this ok?
fn change_mowed_material(
    field_q: Query<(&Field, &Children)>,
    field_square_q: Query<&FieldSquareMarker>,
    mut commands: Commands,
    field_material: Res<FieldMaterialResource>,
) {
    field_q.iter().for_each(|(field, children)| {
        if field.field_type != FieldType::Canvas {
            return;
        }
        children.iter().for_each(|field_square_entity| {
            if let Ok(fs) = field_square_q.get(*field_square_entity) {
                let is_mowed = field
                    .mowed
                    .get(&(fs.0.x as i32, fs.0.y as i32))
                    .unwrap_or(&false);

                if *is_mowed {
                    commands
                        .entity(*field_square_entity)
                        .insert(field_material.mowed.clone());
                }
            }
        });
    });
}

fn mow_target_field(
    harvestor_q: Query<&Harvestor, Changed<Harvestor>>,
    mut field_q: Query<&mut Field>,
) {
    harvestor_q.iter().for_each(|h| {
        field_q.iter_mut().for_each(|mut field| {
            if field.field_type != FieldType::Canvas {
                return;
            }
            field.mowed.insert((h.position.x, h.position.y), true);
        });
    });
}

fn compare_fields_on_commands_cleared(
    mut ev_harvestor_commands_cleared: EventReader<HarvestorCommandsClearedEvent>,
    field_q: Query<&Field>,
    mut commands: Commands,
    font: Res<FontHandle>,
    help_ui_container_q: Query<Entity, With<HelpTextContainer>>,
) {
    for _ in ev_harvestor_commands_cleared.iter() {
        let target_field = field_q.iter().find(|f| f.field_type == FieldType::Target);
        let canvas_field = field_q.iter().find(|f| f.field_type == FieldType::Canvas);

        if let (Some(target), Some(canvas)) = (target_field, canvas_field) {
            let result = compare_fields(target, canvas);

            let result_text = match result {
                MowResult::Perfect => "Success! :)",
                MowResult::TooMuch => "Too many fields are harvested :( ",
                MowResult::TooLittle => "Some fields are not harvested :(",
            };

            let e = help_ui_container_q.single();
            update_help_text(&font, &mut commands, e, result_text);
        }
    }
}

#[derive(PartialEq, Debug)]
enum MowResult {
    Perfect,
    TooMuch,
    TooLittle,
}

fn compare_fields(field_target: &Field, field_canvas: &Field) -> MowResult {
    let _result = false;
    // canvas should not have mowed a field square that's mowed in target

    for (coord, canvas_mowed) in field_canvas.mowed.iter() {
        let target_mowed = field_target.mowed.get(coord).unwrap_or(&false);

        if *canvas_mowed && *target_mowed {
            return MowResult::TooMuch;
        }
    }

    // all target unmowed squared should be mowed in canvas
    for coord in (0..field_target.size.y)
        .flat_map(|x| (0..field_target.size.x).map(move |y| (x as i32, y as i32)))
    {
        let mowed = field_target.mowed.get(&coord).unwrap_or(&false);
        let field_is_mowed = field_canvas.mowed.get(&coord).unwrap_or(&false);
        if !mowed && !field_is_mowed {
            return MowResult::TooLittle;
        }
    }

    MowResult::Perfect
}

#[test]
fn fully_mowed_field() {
    let mut target_mowed = HashMap::new();
    target_mowed.insert((0, 0), true);
    target_mowed.insert((0, 1), true);
    target_mowed.insert((1, 0), true);
    target_mowed.insert((1, 1), true);

    let field_target = Field {
        size: UVec2::new(2, 2),
        mowed: target_mowed,
        ..default()
    };
    let field_canvas = Field {
        size: UVec2::new(2, 2),
        mowed: HashMap::new(),
        ..default()
    };

    assert_eq!(
        compare_fields(&field_target, &field_canvas),
        MowResult::Perfect
    );
}

#[test]
fn fully_unmowed_field() {
    let target_mowed = HashMap::new();
    let field_target = Field {
        size: UVec2::new(2, 2),
        mowed: target_mowed,
        ..default()
    };
    let mut canvas_mowed = HashMap::new();
    canvas_mowed.insert((0, 0), true);
    canvas_mowed.insert((0, 1), true);
    canvas_mowed.insert((1, 0), true);
    canvas_mowed.insert((1, 1), true);

    let field_canvas = Field {
        size: UVec2::new(2, 2),
        mowed: canvas_mowed,
        ..default()
    };

    assert_eq!(
        compare_fields(&field_target, &field_canvas),
        MowResult::Perfect
    );
}

#[test]
fn partial_mowed() {
    let mut target_mowed = HashMap::new();
    target_mowed.insert((0, 0), true);
    target_mowed.insert((0, 1), true);
    target_mowed.insert((1, 1), false);

    let field_target = Field {
        size: UVec2::new(2, 2),
        mowed: target_mowed,
        ..default()
    };
    let mut canvas_mowed = HashMap::new();
    canvas_mowed.insert((1, 0), true);
    canvas_mowed.insert((1, 1), true);

    let field_canvas = Field {
        size: UVec2::new(2, 2),
        mowed: canvas_mowed,
        ..default()
    };

    assert_eq!(
        compare_fields(&field_target, &field_canvas),
        MowResult::Perfect
    );
}

#[test]
fn too_little_mowed() {
    let mut target_mowed = HashMap::new();
    target_mowed.insert((0, 0), true);
    target_mowed.insert((0, 1), true);
    let field_target = Field {
        size: UVec2::new(2, 2),
        mowed: target_mowed,
        ..default()
    };
    let mut canvas_mowed = HashMap::new();
    canvas_mowed.insert((1, 1), true);

    let field_canvas = Field {
        size: UVec2::new(2, 2),
        mowed: canvas_mowed,
        ..default()
    };

    assert_eq!(
        compare_fields(&field_target, &field_canvas),
        MowResult::TooLittle
    );
}

#[test]
fn too_much_mowed() {
    let mut target_mowed = HashMap::new();
    target_mowed.insert((0, 0), true);
    target_mowed.insert((0, 1), true);
    let field_target = Field {
        size: UVec2::new(2, 2),
        mowed: target_mowed,
        ..default()
    };
    let mut canvas_mowed = HashMap::new();
    canvas_mowed.insert((0, 0), true);
    canvas_mowed.insert((1, 1), true);
    canvas_mowed.insert((1, 0), false);

    let field_canvas = Field {
        size: UVec2::new(2, 2),
        mowed: canvas_mowed,
        ..default()
    };

    assert_eq!(
        compare_fields(&field_target, &field_canvas),
        MowResult::TooMuch
    );
}
