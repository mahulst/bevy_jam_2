use bevy::prelude::*;

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(render_fields).add_startup_system(setup);
    }
}

enum FieldType {
    Target,
    Canvas,
}

#[derive(Component)]
pub struct Field {
    size: UVec2,
    field_type: FieldType,
}

fn setup(mut commands: Commands) {
    commands.spawn().insert(Field {
        size: UVec2::new(10, 10),
        field_type: FieldType::Target,
    });
    commands.spawn().insert(Field {
        size: UVec2::new(10, 10),
        field_type: FieldType::Canvas,
    });
}

pub const FIELD_SIZE: f32 = 0.2;
pub const FIELD_MARGIN_SIZE: f32 = 0.01;
pub const FIELD_THICKNESS: f32 = 0.02;
fn render_fields(
    query: Query<(Entity, &Field), (Added<Field>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    query.iter().for_each(|(e, field)| {
        let mut entity = commands.entity(e);
        entity.insert_bundle(SpatialBundle { ..default() });

        (0..=field.size.y)
            .flat_map(|x| {
                (0..=field.size.x).map(move |y| (x as f32 * (FIELD_SIZE + FIELD_MARGIN_SIZE), y as f32 * (FIELD_SIZE + FIELD_MARGIN_SIZE)))
            })
            .for_each(|(x, y)| {
                entity.with_children(|cb| {
                    cb.spawn().insert_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: FIELD_SIZE })),
                        material: materials.add(Color::rgb(0.4, 0.2, 0.0).into()),
                        transform: Transform::from_xyz(x, 0.0, y)
                            .with_scale(Vec3::new(1.0, FIELD_THICKNESS, 1.0)),
                        ..default()
                    });
                });
            });
    });
}
