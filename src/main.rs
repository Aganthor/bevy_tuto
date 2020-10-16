use bevy::{
    prelude::*,
};

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
        .add_default_plugins()
        .add_plugin(map_plugin::MapPlugin)
        .add_startup_system(setup.system())
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(player_movement_system.system())
        .run();
}

fn setup (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
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
            direction: player::Direction::Idle,
         });
}