use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ArrowImage>()
            .init_resource::<FontHandle>()
            .add_startup_system(setup_font)
            .add_startup_system(setup_countdown.after(setup_font))
            .add_startup_system(setup_commands)
            .add_startup_system(setup_win_lose_text.after(setup_font));
    }
}

#[derive(Default)]
pub struct ArrowImage {
    pub(crate) handle: Handle<Image>,
}

#[derive(Default)]
pub struct FontHandle {
    pub(crate) handle: Handle<Font>,
}
fn setup_font(asset_server: Res<AssetServer>, mut font_handle: ResMut<FontHandle>) {
    let handle = asset_server.load("fonts/FiraSans-Bold.ttf");
    font_handle.handle = handle;
}

#[derive(Component)]
pub struct CountDownMarkerSeconds;
#[derive(Component)]
pub struct CountDownMarkerMilliSeconds;

fn setup_countdown(mut commands: Commands, font: Res<FontHandle>) {
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
                                    font: font.handle.clone(),
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
                                            font: font.handle.clone(),
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

#[derive(Component)]
pub struct HelpTextContainer;

fn setup_win_lose_text(mut commands: Commands, font: Res<FontHandle>) {
    let mut ui_entity = None;
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
            let entity = parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(20.0)),
                        margin: UiRect {
                            left: Val::Undefined,
                            right: Val::Undefined,
                            top: Val::Undefined,
                            bottom: Val::Percent(10.0),
                        },
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::all(Val::Px(16.0)),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                })
                .insert(HelpTextContainer)
                .id();

            ui_entity = Some(entity);
        });

    if let Some(e) = ui_entity {
        update_help_text(
            &font,
            &mut commands,
            e,
            "Press any arrow key to insert command.",
        );
    };
}

pub fn update_help_text(
    font: &Res<FontHandle>,
    commands: &mut Commands,
    ui_entity: Entity,
    text: &str,
) {
    let mut help_ui_container = commands.entity(ui_entity);
    help_ui_container.despawn_descendants();

    help_ui_container.with_children(|p| {
        p.spawn_bundle(
            TextBundle::from_section(
                text,
                TextStyle {
                    font: font.handle.clone(),
                    font_size: 32.0,
                    color: Color::WHITE,
                },
            )
            .with_style(Style {
                margin: UiRect::all(Val::Px(5.0)),
                ..default()
            }),
        );
    });
}
