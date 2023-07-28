use bevy::prelude::*;
use bevy::input::keyboard::*;
use rand::Rng;

#[derive(Resource, Default)]
struct Game {
    board: Vec<Vec<Cell>>,
}

struct Cell {}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .add_systems(Startup, setup)
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

fn setup(
    mut commands: Commands,
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
