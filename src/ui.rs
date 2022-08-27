use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_countdown)
            .init_resource::<ArrowImage>()
            .add_startup_system(setup_commands);
    }
}

#[derive(Default)]
pub struct ArrowImage {
    pub(crate) handle: Handle<Image>,
}

#[derive(Component)]
pub struct CountDownMarkerSeconds;
#[derive(Component)]
pub struct CountDownMarkerMilliSeconds;

fn setup_countdown(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .with_children(|countdown_node| {
                    countdown_node
                        .spawn_bundle(
                            TextBundle::from_section(
                                "1",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 64.0,
                                    color: Color::WHITE,
                                },
                            )
                            .with_style(Style {
                                margin: UiRect::all(Val::Px(5.0)),
                                ..default()
                            }),
                        )
                        .insert(CountDownMarkerSeconds);
                    countdown_node
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size {
                                    height: Val::Percent(100.0),
                                    width: Val::Px(50.0),
                                },
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::FlexEnd,
                                ..default()
                            },
                            color: Color::NONE.into(),
                            ..default()
                        })
                        .with_children(|countdown_node| {
                            countdown_node
                                .spawn_bundle(
                                    TextBundle::from_section(
                                        "99",
                                        TextStyle {
                                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                            font_size: 32.0,
                                            color: Color::WHITE,
                                        },
                                    )
                                    .with_style(Style {
                                        margin: UiRect::all(Val::Px(5.0)),
                                        ..default()
                                    }),
                                )
                                .insert(CountDownMarkerMilliSeconds);
                        });
                });
        });
}

#[derive(Component)]
pub struct CommandsContainerMarker;

fn setup_commands(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut arrow_image: ResMut<ArrowImage>,
) {
    let handle = asset_server.load("arrow.png");
    arrow_image.handle = handle;
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::FlexStart,
                position_type: PositionType::Absolute,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                        flex_direction: FlexDirection::Row,
                        flex_wrap: FlexWrap::WrapReverse,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexEnd,
                        padding: UiRect::all(Val::Px(16.0)),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .insert(CommandsContainerMarker);
        });
}
