use bevy::prelude::*;

mod player;
use player::*;

mod events;
use events::GameEvent;

mod plugins;
use crate::plugins::map::*;

fn main() {
    App::build()
        .add_event::<GameEvent>()
        .add_resource(WindowDescriptor {
            title: "Void destiny - the roguelike game".to_string(),
            width: 1024,
            height: 768,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(map_plugin::MapPlugin)
        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(player_movement_system)
        .add_system(mouse_movement_updating_system)
        .add_system(get_tile_info_system)
        .run();
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_player(commands, &asset_server, &mut materials);
}
