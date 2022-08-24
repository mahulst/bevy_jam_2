use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_inspector_egui::{Inspectable, RegisterInspectable};

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_fields)
            .add_startup_system(setup)
            .register_inspectable::<Field>();
    }
}

#[derive(Inspectable)]
enum FieldType {
    Target,
    Canvas,
}

#[derive(Component, Inspectable)]
pub struct Field {
    size: UVec2,
    field_type: FieldType,
    #[inspectable(ignore)]
    mowed: HashMap<(u32, u32), bool>,
}

fn setup(mut commands: Commands) {
    let mut target_mowed = HashMap::new();
    target_mowed.insert((0, 0), true);
    target_mowed.insert((0, 1), true);
    target_mowed.insert((0, 2), true);
    target_mowed.insert((0, 3), true);
    target_mowed.insert((0, 4), true);
    target_mowed.insert((1, 4), true);
    target_mowed.insert((2, 4), true);
    target_mowed.insert((3, 4), true);
    target_mowed.insert((4, 4), true);
    target_mowed.insert((4, 5), true);
    target_mowed.insert((4, 6), true);
    target_mowed.insert((4, 7), true);
    target_mowed.insert((4, 8), true);
    target_mowed.insert((5, 8), true);
    target_mowed.insert((6, 8), true);
    target_mowed.insert((7, 8), true);
    target_mowed.insert((8, 8), true);
    target_mowed.insert((8, 9), true);
    target_mowed.insert((9, 9), true);
    target_mowed.insert((9, 10), true);
    target_mowed.insert((10, 10), true);
    commands.spawn().insert(Field {
        size: UVec2::new(10, 10),
        field_type: FieldType::Target,
        mowed: HashMap::new(),
    });
    commands.spawn().insert(Field {
        size: UVec2::new(10, 10),
        field_type: FieldType::Canvas,
        mowed: target_mowed,
    });
}

const FIELD_FRESH_COLOR: Color = Color::rgb(0.536, 0.389, 0.076);
const FIELD_MOWED_COLOR: Color = Color::rgb(0.4, 0.2, 0.0);

pub const FIELD_SIZE: f32 = 0.2;
pub const FIELD_MARGIN_SIZE: f32 = 0.01;
pub const FIELD_THICKNESS: f32 = 0.02;
fn render_fields(
    query: Query<(Entity, &Field), Added<Field>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = Mesh::from(shape::Cube { size: FIELD_SIZE });
    let handle = meshes.add(mesh);

    let material_field_fresh = materials.add(FIELD_FRESH_COLOR.into());
    let material_field_mowed = materials.add(FIELD_MOWED_COLOR.into());

    query.iter().for_each(|(e, field)| {
        let pos = match field.field_type {
            FieldType::Target => -Vec3::X,
            FieldType::Canvas => Vec3::X,
        };
        let pos = pos * 2.0 - Vec3::X;

        let mut entity = commands.entity(e);
        entity.insert_bundle(SpatialBundle { ..default() });

        (0..=field.size.y)
            .flat_map(|x| (0..=field.size.x).map(move |y| (x, y)))
            .for_each(|(x, y)| {
                let material = if field.mowed.contains_key(&(x as u32, y as u32)) {
                    material_field_mowed.clone()
                } else {
                    material_field_fresh.clone()
                };

                let world_x = x as f32 * (FIELD_SIZE + FIELD_MARGIN_SIZE);
                let world_y = y as f32 * (FIELD_SIZE + FIELD_MARGIN_SIZE);
                entity.with_children(|cb| {
                    cb.spawn().insert_bundle(PbrBundle {
                        mesh: handle.clone(),
                        material,
                        transform: Transform::from_xyz(world_x + pos.x, 0.0, world_y)
                            .with_scale(Vec3::new(1.0, FIELD_THICKNESS, 1.0)),
                        ..default()
                    });
                });
            });
    });
}