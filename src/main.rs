use bevy::prelude::*;

mod player;
use player::*;

mod plugins;
use crate::plugins::map::*;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "Void destiny - the roguelike game".to_string(),
            width: 800,
            height: 600,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(map_plugin::MapPlugin)
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(player_movement_system.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dComponents::default());
    spawn_player(&mut commands, &asset_server, &mut materials);
}
