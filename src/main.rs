use bevy::{
    input::keyboard::*,
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};
use rand::Rng;

#[derive(Resource, Default)]
struct Game {
    board: Vec<Vec<Wall>>,
}

#[derive(Component)]
struct Wall;

// Marker for the player
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health {
    current: usize,
    max: usize,
}

#[derive(Component)]
struct Location {
    i: f32,
    j: f32,
}

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .add_event::<CollisionEvent>()
        .add_systems(Startup, (setup, spawn_player, spawn_enemy))
        .add_systems(Update, (keyboard_input_system, move_player))
        .add_systems(FixedUpdate, check_for_collisions)
        .run();
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

    (0..h as i32).step_by(10).for_each(|jint| {
        let j = jint as f32;

        (0..w as i32).step_by(10).for_each(|iint| {
            let i = iint as f32;

            commands.spawn((
                // give it a marker
                Wall,
                // give it a 2D sprite to render on-screen
                // (Bevy's SpriteBundle lets us add everything necessary)
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.98, 0.5, 0.45),
                        custom_size: Some(Vec2::new(8.5, 8.5)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(i - w / 2 as f32, j - h / 2 as f32, 0.),
                    // use the default values for all other components in the bundle
                    ..Default::default()
                },
                Collider,
            ));
        })
    })
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
        Location { i: 25.0, j: 50.0 },
        Health {
            current: 100,
            max: 125,
        },
        Speed(100.0),
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

fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: Query<&mut Window>,
) {
    let mut window = windows.single_mut();
    let h = window.height();
    let w = window.width();
    let mut rng = rand::thread_rng();
    let start_x = rng.gen_range(0.0..(w / 2 as f32));
    let start_y = rng.gen_range(0.0..(h / 2 as f32));

    // create a new entity with whatever components we want
    commands.spawn((
        // give it a marker
        Enemy,
        // give it health and xp
        Health {
            current: 100,
            max: 125,
        },
        // give it a 2D sprite to render on-screen
        // (Bevy's SpriteBundle lets us add everything necessary)
        SpriteBundle {
            texture: asset_server.load("ghost1.png"),
            transform: Transform::from_xyz(start_x, start_y, 0.0),
            // use the default values for all other components in the bundle
            ..Default::default()
        },
    ));
}

fn check_for_collisions(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    collider_query: Query<(Entity, &Transform, Option<&Wall>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let player_transform = player_query.single();
    let player_size = player_transform.scale.truncate() * 5.0;

    // check collision with walls
    for (collider_entity, transform, maybe_wall) in &collider_query {
        let collision = collide(
            player_transform.translation,
            player_size,
            transform.translation,
            transform.scale.truncate() * 5.0,
        );
        if let Some(collision) = collision {
            // Sends a collision event so that other systems can react
            collision_events.send_default();

            // Walls should be despawned
            if maybe_wall.is_some() {
                commands.entity(collider_entity).despawn();
            }
        }
    }
}

/// This system prints out all keyboard events as they come in
fn print_keyboard_event_system(mut keyboard_input_events: EventReader<KeyboardInput>) {
    for event in keyboard_input_events.iter() {
        info!("{:?}", event);
    }
}

/// This system prints 'A' key state
fn keyboard_input_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Location, &Speed), With<Player>>,
) {
    let (mut loc, speed) = player_query.single_mut();

    if keyboard_input.pressed(KeyCode::Up) {
        loc.j += speed.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Down) {
        loc.j -= speed.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Left) {
        loc.i -= speed.0 * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::Right) {
        loc.i += speed.0 * time.delta_seconds();
    }
}

fn move_player(mut player_query: Query<(&mut Transform, &Location), With<Player>>) {
    let (mut transform, location) = player_query.single_mut();
    transform.translation.x = location.i;
    transform.translation.y = location.j;
}
