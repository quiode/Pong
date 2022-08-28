use bevy::{
    prelude::*,
    window::{close_on_esc, WindowMode},
};
use bevy_prototype_lyon::prelude::*;

const WIDTH: f32 = 852.0;
const HEIGHT: f32 = 480.0;

const BALL_RADIUS: f32 = 8.0;

// main function
fn main() {
    // When building for WASM, print panics to the browser console
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .insert_resource(WindowDescriptor {
            title: "Pong".to_string(),
            cursor_visible: false,
            mode: WindowMode::Windowed,
            width: WIDTH,
            height: HEIGHT,
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
        .add_system(controls)
        .add_system(out_of_bounds_detector)
        .add_system(collision_detector)
        .add_system(scoreboard_changer)
        .add_event::<ScoreEvent>()
        .run();
}

/// component for the score text
#[derive(Component)]
struct ScoreText;

/// component for everything that has a score
#[derive(Component)]
struct Score(u32);

/// position of the player
#[derive(Default)]
enum PlayerPosition {
    #[default]
    Left,
    Right,
}
/// component for a player
#[derive(Component)]
struct Player(PlayerPosition);

/// a ball
#[derive(Component)]
struct Ball;

/// mass, gets influenced by gravity
#[derive(Component)]
struct Mass(f32);

/// velocity
#[derive(Component)]
struct Velocity {
    x: f32,
    y: f32,
}

/// score change event, PlayerPosition determines which player scored the point
#[derive(Default)]
struct ScoreEvent(PlayerPosition);

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
        radius: BALL_RADIUS,
        ..default()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform::default(),
        ))
        .insert(Mass(10.0))
        .insert(Velocity { x: 50.0, y: 0.0 })
        .insert(Ball);
}

// spawns the two players
fn spawn_players(mut commands: Commands) {
    let shape = shapes::Rectangle {
        extents: Vec2 { x: 5.0, y: 200.0 },
        ..default()
    };

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform::from_translation(Vec3 {
                x: -410.0,
                ..default()
            }),
        ))
        .insert(Player(PlayerPosition::Left));

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Fill(FillMode::color(Color::WHITE)),
            Transform::from_translation(Vec3 {
                x: 410.0,
                ..default()
            }),
        ))
        .insert(Player(PlayerPosition::Right));
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

// reacts to keyboard input
fn controls(
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&Player, &mut Transform)>,
    time: Res<Time>,
) {
    const SPEED: f32 = 100.0;
    const BORDER_STOP: f32 = 290.0;

    for (player, mut transform) in query.iter_mut() {
        match player.0 {
            PlayerPosition::Left => {
                if keys.pressed(KeyCode::W) && transform.translation.y + BORDER_STOP < WIDTH / 2.0 {
                    transform.translation.y += SPEED * time.delta_seconds()
                } else if keys.pressed(KeyCode::S)
                    && transform.translation.y - BORDER_STOP > -(WIDTH / 2.0)
                {
                    transform.translation.y -= SPEED * time.delta_seconds()
                }
            }
            PlayerPosition::Right => {
                if keys.pressed(KeyCode::Up) && transform.translation.y + BORDER_STOP < WIDTH / 2.0
                {
                    transform.translation.y += SPEED * time.delta_seconds()
                } else if keys.pressed(KeyCode::Down)
                    && transform.translation.y - BORDER_STOP > -(WIDTH / 2.0)
                {
                    transform.translation.y -= SPEED * time.delta_seconds()
                }
            }
        }
    }
}

/// checks if the ball is out of bounds and if so emit a score event and reset the ball
fn out_of_bounds_detector(
    mut query: Query<(&Ball, &mut Transform)>,
    mut event: EventWriter<ScoreEvent>,
) {
    let (_ball, mut transform) = query.single_mut();

    // check and emit event
    if transform.translation.x > WIDTH / 2.0 {
        event.send(ScoreEvent(PlayerPosition::Right))
    } else if transform.translation.x < -(WIDTH / 2.0) {
        event.send(ScoreEvent(PlayerPosition::Left))
    } else {
        // exit so the next steps don't apply if the ball isn't out of bounds
        return;
    }

    // recenter ball
    transform.translation.x = 0.0;
    transform.translation.y = 0.0;
}

/// checks if the ball collides with either the top, bottom or one of the players and if so, reflect the ball
fn collision_detector(mut query: Query<(&Ball, &Transform, &mut Mass)>) {
    let (_ball, transform, mut mass) = query.single_mut();

    if transform.translation.y + BALL_RADIUS > HEIGHT / 2.0
        || transform.translation.y - BALL_RADIUS < -(HEIGHT / 2.0)
    {
        mass.0 *= -1.0;
    }
}

/// listen for score changes and react to them
fn scoreboard_changer(event: EventReader<ScoreEvent>) {
    // todo!()
}
