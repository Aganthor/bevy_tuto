use bevy::{
    asset::{HandleId, LoadState, Handle},
    prelude::*,
    sprite::TextureAtlasBuilder,
};

use super::map_creator::*;

pub struct MapPlugin;

#[derive(Default)]
pub struct MapSpriteHandles {
    handles: Vec<HandleUntyped>,
    atlas_loaded: bool,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MapSpriteHandles>()
            .init_resource::<Map>()
            .init_resource::<TileData>()
            .add_startup_system(setup.system())
            .add_system(spawn_map.system());
    }
}

fn setup(mut map_sprite_handles: ResMut<MapSpriteHandles>, asset_server: Res<AssetServer>) {
    map_sprite_handles.handles = asset_server.load_folder("map_tiles/").unwrap();
}

//TODO: Faire un meilleur découpage des system dans le plugin.
//Avoir un systeme "load_map" et un system "spawn_map" qui s'occupe de l'affichage.
fn spawn_map(
    mut commands: Commands,
    mut map_sprite_handles: ResMut<MapSpriteHandles>,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    tile_data: Res<TileData>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if !map_sprite_handles.atlas_loaded {
        let mut texture_atlas_builder = TextureAtlasBuilder::default();
        if let LoadState::Loaded =
            asset_server.get_group_load_state(map_sprite_handles.handles.iter().map(|handle| handle.id))
        {
            for texture_id in map_sprite_handles.handles.iter() {
                let texture = textures.get(texture_id).unwrap();
                texture_atlas_builder.add_texture(texture_id.clone_weak().typed::<Texture>(), &texture);
            }

            let mut texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

            create_map(
                &mut commands,
                &map,
                &tile_data,
                &asset_server,
                &mut texture_atlas,
                &mut materials
            );

            map_sprite_handles.atlas_loaded = true;
        }
    }
}
