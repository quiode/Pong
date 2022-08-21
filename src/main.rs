use bevy::{
    prelude::*,
    window::{close_on_esc, WindowMode},
};
use bevy_prototype_lyon::prelude::*;

// main function
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            cursor_visible: false,
            mode: WindowMode::Windowed,
            width: 852.0,
            height: 480.0,
            resizable: false,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa {
            ..Default::default()
        })
        .add_startup_system(spawn_ball)
        .add_startup_system(spawn_players)
        .add_system(gravity)
        .add_system(velocity)
        .add_system(close_on_esc)
        .run();
}

/// component for the score text
#[derive(Component)]
struct ScoreText;

/// component for everything that has a score
#[derive(Component)]
struct Score(u32);

/// component for a player
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ball;

// mass, gets influenced by gravity
#[derive(Component)]
struct Mass(f32);

// velocity
#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

/// setup
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());
    // Text
    let text_style = TextStyle {
        font: asset_server.load("fonts/RetroGaming.ttf"),
        font_size: 50.0,
        color: Color::WHITE,
    };

    // create the scoreboard
    commands
        .spawn_bundle(
            TextBundle::from_sections([
                TextSection::new("0", text_style.clone()),
                TextSection::new(":", text_style.clone()),
                TextSection::new("0", text_style.clone()),
            ])
            // .with_text_alignment(TextAlignment::TOP_CENTER)
            .with_style(Style {
                align_self: AlignSelf::Center,
                margin: UiRect {
                    top: Val::Px(0.0),
                    bottom: Val::Auto,
                    left: Val::Auto,
                    right: Val::Auto,
                },
                ..default()
            }),
        )
        .insert(ScoreText);
}

// spawns the ball
fn spawn_ball(mut commands: Commands) {
    let shape = shapes::Circle {
        radius: 8.0,
        ..default()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform::default(),
        ))
        .insert(Mass(10.0))
        .insert(Velocity { x: 50.0, y: 0.0 });
}

// spawns the two players
fn spawn_players(mut commands: Commands) {
    let shape = shapes::Rectangle {
        extents: Vec2 { x: 5.0, y: 200.0 },
        ..default()
    };

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Fill(FillMode::color(Color::WHITE)),
        Transform::from_translation(Vec3 {
            x: -410.0,
            ..default()
        }),
    ));

    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        DrawMode::Fill(FillMode::color(Color::WHITE)),
        Transform::from_translation(Vec3 {
            x: 410.0,
            ..default()
        }),
    ));
}

// function that applies gravity to objects that have a mass
fn gravity(mut query: Query<(&Mass, &mut Transform)>, time: Res<Time>) {
    const G: f32 = 9.81;

    for (mass, mut transform) in query.iter_mut() {
        transform.translation.y -= time.delta_seconds() * G * mass.0;
    }
}

// applies velocity
fn velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.y += velocity.y * time.delta_seconds();
        transform.translation.x += velocity.x * time.delta_seconds();
    }
}
