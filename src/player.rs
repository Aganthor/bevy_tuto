use bevy::{
    input::{mouse::MouseButtonInput, ElementState},
    prelude::*,
    render::camera::{OrthographicProjection, WindowOrigin},
};

use crate::plugins::map::map_creator::Map;
use crate::plugins::map::map_creator::TILE_SIZE;

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
    pub cursor: EventReader<CursorMoved>,
    pub button: EventReader<MouseButtonInput>,
    pub camera_e: Entity,
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
        ..Default::default()
    };

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
                5.0,
            )),
            ..Default::default()
        })
        .with(Player {
            speed: TILE_SIZE as f32,
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
        mouse_pos.0 = transform_pos_to_map_pos(&event.position.extend(5.0)).truncate();
    }
}

pub fn get_tile_info_system(
    ev_button: Res<Events<MouseButtonInput>>,
    map: Res<Map>,
    mouse_pos: Res<MouseLocation>,
    mut state: ResMut<CursorState>,
) {
    for event in state.button.iter(&ev_button) {
        if event.state == ElementState::Pressed {
            let tile_info = map.get_tileinfo_at(mouse_pos.0.x as usize, mouse_pos.0.y as usize);
            info!("The terrain type is {}", tile_info.tile_type);
        }
    }
}

fn transform_pos_to_map_pos(position: &Vec3) -> Vec3 {
    Vec3::new(
        (position.x / TILE_SIZE as f32).floor(),
        (position.y / TILE_SIZE as f32).floor(),
        5.0,
    )
}

pub fn player_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    windows: Res<Windows>,
    map: Res<Map>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    for (mut player, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        let mut player_destination: Vec3 = Vec2::zero().extend(5.0);

        if keyboard_input.just_pressed(KeyCode::Left) {
            player.direction = Direction::Left;
            player_destination.x = translation.x + player.speed * -1.0;
            player_destination.y = translation.y;
        } else if keyboard_input.just_pressed(KeyCode::Right) {
            player.direction = Direction::Right;
            player_destination.x = translation.x + player.speed * 1.0;
            player_destination.y = translation.y;
        } else if keyboard_input.just_pressed(KeyCode::Up) {
            player.direction = Direction::Up;
            player_destination.x = translation.x;
            player_destination.y = translation.y + player.speed * 1.0;
        } else if keyboard_input.just_pressed(KeyCode::Down) {
            player.direction = Direction::Down;
            player_destination.x = translation.x;
            player_destination.y = translation.y + player.speed * -1.0;
        } else {
            player.direction = Direction::Idle;
            return;
        }

        let active_window = windows.get_primary().unwrap();

        let movement: (bool, bool) =
            validate_movement(&player_destination, &player.direction, &map, &active_window);
        if movement.0 {
            if movement.1 {
                //Movement is legal, proceed.
                translation.x = player_destination.x;
                translation.y = player_destination.y;
            } else {
                info!("Movement was illegal...");
                return;
            }
        } else {
            info!("Movement was illegal...");
            return;
        }
    }
}

fn validate_movement(
    player_destination: &Vec3,
    direction: &Direction,
    map: &Res<Map>,
    window: &Window,
) -> (bool, bool) {
    let mut screen_movement_legal = false;
    let mut map_terrain_movement_legal = false;

    //First, check if the player wants to move outside the game screen.
    match direction {
        Direction::Left => {
            if player_destination.x < 0.0 {
                screen_movement_legal = false;
            } else {
                screen_movement_legal = true;
            }
        }
        Direction::Right => {
            if player_destination.x < window.width() as f32 {
                screen_movement_legal = true;
            }
        }
        Direction::Up => {
            if player_destination.y < window.height() as f32 {
                screen_movement_legal = true;
            }
        }
        Direction::Down => {
            if player_destination.y > 0.0 {
                screen_movement_legal = true;
            }
        }
        Direction::Idle => {}
    }

    if !screen_movement_legal {
        return (false, false);
    }

    //Second, check whether the ground tile is walkable at the player_destination.
    let map_pos = transform_pos_to_map_pos(&player_destination);
    let tile_info = map.get_tileinfo_at(map_pos.x as usize, map_pos.y as usize);
    map_terrain_movement_legal = tile_info.walkable;

    (screen_movement_legal, map_terrain_movement_legal)
}
