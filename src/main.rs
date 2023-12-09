use bevy::prelude::*;
use bevy::input::keyboard::*;
use rand::Rng;

#[derive(Resource, Default)]
struct Game {
    board: Vec<Vec<Cell>>,
}

#[derive(Component)]
struct Cell;

// Marker for the player
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Health {
    current: usize,
    max: usize,
}

// Marker for the player
#[derive(Component)]
struct Xp(usize);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, move_players)
        .add_systems(
            Update,
            (
                print_keyboard_event_system,
                keyboard_input_system,
            )
        )
        .run();
}

/// This system prints out all keyboard events as they come in
fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.iter() {
        info!("{:?}", event);
    }
}

/// This system prints 'A' key state
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::Left) {
        info!("'Left' pressed");
    }

    if keyboard_input.pressed(KeyCode::Down) {
        info!("'down' pressed");
    }

    if keyboard_input.just_pressed(KeyCode::Right) {
        info!("'Right' just pressed");
    }

    if keyboard_input.just_released(KeyCode::Up) {
        info!("'Up' just released");
    }
}

fn move_players(
    time: Res<Time>,
    mut q: Query<&mut Transform, With<Player>>,
) {
    for mut transform in q.iter_mut() {
        // move our asteroids along the X axis
        // at a speed of 10.0 units per second
        transform.translation.x += 10.0 * time.delta_seconds();
    }
}

fn spawn_player(
    // needed for creating/removing data in the ECS World
    mut commands: Commands,
    // needed for loading assets
    asset_server: Res<AssetServer>,
) {
    // create a new entity with whatever components we want
    commands.spawn((
        // give it a marker
        Player,
        // give it health and xp
        Health {
            current: 100,
            max: 125,
        },
        Xp(0),
        // give it a 2D sprite to render on-screen
        // (Bevy's SpriteBundle lets us add everything necessary)
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(25.0, 50.0, 0.0),
            // use the default values for all other components in the bundle
            ..Default::default()
        },
    ));
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game: ResMut<Game>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();

    commands.spawn(Camera2dBundle::default());

    let h = window.height();
    let w = window.width();

    game.board = (0..h as i32)
        .step_by(10)
        .map(|jint| {
            let j = jint as f32;

            (0..w as i32)
                .step_by(10)
                .map(|iint| {
                    let i = iint as f32;

                    commands.spawn(SpriteBundle {
                        sprite: Sprite {
                          color: Color::rgb(0.98, 0.5, 0.45),
                          custom_size: Some(Vec2::new(8.5, 8.5)),
                          ..default()
                        },
                        transform: Transform::from_xyz(i - w/2 as f32, j - h/2 as f32, 0.),
                        ..default()
                    });
                    Cell { }
                })
                .collect()
        })
        .collect();
}
