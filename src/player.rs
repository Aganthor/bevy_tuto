use bevy::{render::camera::{
    OrthographicProjection, WindowOrigin},
    prelude::*
};

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

pub struct CursorState {
    cursor: EventReader<CursorMoved>,
    camera_e: Entity,
}

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
        camera_e: e,
    });

    let texture_handle = asset_server.load("unseen_horror_new.png");
    commands
        .spawn(SpriteBundle {
            material: materials.add(texture_handle.into()),
            transform: Transform::from_translation(Vec3::new(32.0, 1.0, 5.0)),
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
    mut state: ResMut<CursorState>,
    ev_cursor: Res<Events<CursorMoved>>,
    //q_camera: Query<&Transform>,
    mut query: Query<(&mut Player, &mut Transform)>,
) {
    //Check world pos...
    // let camera_transform = q_camera.get(state.camera_e).unwrap();

    // for ev in state.cursor.iter(&ev_cursor) {
    //     let wnd = windows.get(ev.id).unwrap();
    //     let size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

    //     let p = ev.position - size;

    //     let world_pos = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
    //     println!("World coords: {}/{}", world_pos.x, world_pos.y);
    // }

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
