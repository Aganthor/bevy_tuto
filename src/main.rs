use bevy::{prelude::*, window::WindowMode};

mod player;
use player::*;

mod events;
use events::GameEvent;

mod plugins;
use crate::plugins::map::*;

fn main() {
    App::build()
        .add_event::<GameEvent>()
        .insert_resource(WindowDescriptor {
            title: "Void destiny - the roguelike game".to_string(),
            width: 1024 as f32,
            height: 768 as f32,
            vsync: true,
            mode: WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(map_plugin::MapPlugin)
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(main_input_system.system())
        .add_system(player_movement_system.system())
        .add_system(mouse_movement_updating_system.system())
        .add_system(get_tile_info_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_player(commands, &asset_server, &mut materials);
}

fn main_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut windows: ResMut<Windows>,
) {  
    if keyboard_input.just_pressed(KeyCode::F) {
        let window = windows.get_primary_mut().unwrap();
        let mode = window.mode();
        match mode {
            WindowMode::BorderlessFullscreen => window.set_mode(WindowMode::Windowed),
            WindowMode::Windowed => window.set_mode(WindowMode::BorderlessFullscreen),
            _ => return
        }
    }
}