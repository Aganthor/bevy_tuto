use bevy::{input::{ElementState, mouse::{MouseButtonInput}}, prelude::*, render::camera::{OrthographicProjection, WindowOrigin}};

use crate::plugins::map::map_creator::TILE_SIZE;
use crate::plugins::map::map_creator::Map;

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

pub struct CursorState {
    cursor: EventReader<CursorMoved>,
    button: EventReader<MouseButtonInput>,
    camera_e: Entity,
}
pub struct MouseLocation(Vec2);

pub fn spawn_player(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let camera = Camera2dBundle {
            orthographic_projection: OrthographicProjection {
                window_origin: WindowOrigin::BottomLeft,
                ..Default::default()
            },
            ..Default::default()};

    let e = commands.spawn(camera).current_entity().unwrap();
    commands.insert_resource(CursorState {
        cursor: Default::default(),
        button: Default::default(),
        camera_e: e,
    });

    let texture_handle = asset_server.load("unseen_horror_new.png");
    commands
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(
                TILE_SIZE as f32 / 2.0, 
                TILE_SIZE as f32 / 2.0, 
                5.0)),
            ..Default::default()
        })
        .with(Player {
            speed: 500.0,
            direction: Direction::Idle,
        });

    commands.insert_resource(MouseLocation(Vec2::new(0.0, 0.0)));
}

pub fn mouse_movement_updating_system(
    mut mouse_pos: ResMut<MouseLocation>,
    mut state: ResMut<CursorState>,
    cursor_moved_events: Res<Events<CursorMoved>>,
) {
    for event in state.cursor.iter(&cursor_moved_events) {
        mouse_pos.0.x = f32::from(event.position.x / TILE_SIZE as f32).floor() + 1.0;
        mouse_pos.0.y = f32::from(event.position.y / TILE_SIZE as f32).floor() + 1.0;
    }
}

pub fn get_tile_info_system(
    ev_button: Res<Events<MouseButtonInput>>,
    map: Res<Map>,
    mouse_pos: Res<MouseLocation>,    
    mut state: ResMut<CursorState>,
) {
    for event in state.button.iter(&ev_button)
    {
        if event.state == ElementState::Pressed {
            let tile_info = map.get_tileinfo_at(mouse_pos.0.x as usize, mouse_pos.0.y as usize);
            println!("The player is standing on {}", tile_info.tile_type);
        }
    }
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
                if player_destination + translation.x < 0.0 {
                    translation.x += player_destination;
                }
            }
            Direction::Right => {
                if player_destination + translation.x < active_window.width() as f32 {
                    translation.x += player_destination;
                }
            }
            Direction::Up => {
                if player_destination + translation.y < active_window.height() as f32 {
                    translation.y += player_destination;
                }
            }
            Direction::Down => {
                if player_destination + translation.y > 0.0 {
                    translation.y += player_destination;
                }
            }
            Direction::Idle => {}
        }
    }
}
