use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*, render::view::RenderLayers};

fn main() {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "UI Test".to_string(),
        width: 1280.0,
        height: 720.0,
        ..default()
    })
    .add_startup_system(setup)
    .add_plugins(DefaultPlugins);

    app.run();
}

fn setup(mut commands: Commands) {
    // ui camera
    let mut ui_camera = Camera2dBundle::default();
    ui_camera.camera.priority = 1;

    commands
        .spawn_bundle(ui_camera)
        .insert(RenderLayers::layer(0));

    // sprite camera
    let mut sprite_camera = Camera2dBundle::default();
    sprite_camera.camera.priority = 2;
    sprite_camera.camera_2d.clear_color = ClearColorConfig::None;

    commands
        .spawn_bundle(sprite_camera)
        .insert(RenderLayers::layer(1))
        .insert(UiCameraConfig { show_ui: false });

    // ui element
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
                margin: UiRect::all(Val::Auto),
                ..default()
            },
            color: Color::RED.into(),
            ..default()
        })
        .insert(RenderLayers::layer(0));

    // sprite
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..default()
            },
            ..default()
        })
        .insert(RenderLayers::layer(1));
}
