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
    let texture_handle = asset_server.load("unseen_horror_new.png");
    commands
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
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
    windows: Res<Windows>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    for (mut player, mut transform) in query.iter_mut() {
        let mut direction = 0.0;
        let translation = &mut transform.translation;

        if keyboard_input.pressed(KeyCode::Left) {
            player.direction = Direction::Left;
            direction -= 1.0;
        }
        else if keyboard_input.pressed(KeyCode::Right) {
            player.direction = Direction::Right;
            direction += 1.0;
        }
        else if keyboard_input.pressed(KeyCode::Up) {
            player.direction = Direction::Up;
            direction += 1.0;
        }
        else if keyboard_input.pressed(KeyCode::Down) {
            player.direction = Direction::Down;
            direction -= 1.0;
        }
        else {
            player.direction = Direction::Idle;
        }

        let active_window = windows.get_primary().unwrap();
        let player_destination = time.delta_seconds * direction * player.speed;

        match player.direction {
            Direction::Left => {
                if player_destination + translation.x > active_window.width() as f32 / -2.0 {
                    translation.x += player_destination;
                }
            }
            Direction::Right => {
                if player_destination + translation.x < active_window.width() as f32 / 2.0 {
                    translation.x += player_destination;
                }
            }
            Direction::Up => {
                if player_destination + translation.y < active_window.height() as f32 / 2.0 {
                    translation.y += player_destination;
                }
            }
            Direction::Down => {
                if player_destination + translation.y > active_window.height() as f32 / -2.0 {
                    translation.y += player_destination;
                }
            }
            Direction::Idle => {}
        }
    }
}
