use bevy::prelude::*;

//use crate::plugins::map::map_creator::TILE_SIZE;

pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    Idle,
}

pub struct Player {
    pub speed: f32,
    pub direction: Direction,
}

pub fn spawn_player(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let texture_handle = asset_server.load("assets/unseen_horror_new.png").unwrap();
    commands
        .spawn(Camera2dComponents::default())
        .spawn(SpriteComponents {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 1.0)),
            ..Default::default()
        })
        .with(Player {
            speed: 500.0,
            direction: Direction::Idle,
        });
}

pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    window: Res<Window>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    for (mut player, mut transform) in &mut query.iter() {
        let mut direction = 0.0;
        let translation = transform.translation_mut();

        if keyboard_input.pressed(KeyCode::Left) {
            player.direction = Direction::Left;
            direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            player.direction = Direction::Right;
            direction += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            player.direction = Direction::Up;
            direction += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            player.direction = Direction::Down;
            direction -= 1.0;
        }

        match player.direction {
            Direction::Left | Direction::Right => {
                *translation.x_mut() += time.delta_seconds * direction * player.speed
            }
            Direction::Up | Direction::Down => {
                *translation.y_mut() += time.delta_seconds * direction * player.speed
            }
            Direction::Idle => {}
        }
    }
}
