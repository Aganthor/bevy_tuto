use bevy::{
    asset::{HandleId, LoadState},
    prelude::*,
    sprite::TextureAtlasBuilder,
};

use super::map_creator::*;

pub struct MapPlugin;

#[derive(Default)]
struct MapSpriteHandles {
    handles: Vec<HandleId>,
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
    map_sprite_handles.handles = asset_server.load_asset_folder("assets/map_tiles").unwrap();
}

fn spawn_map(
    mut commands: Commands,
    mut map_sprite_handles: ResMut<MapSpriteHandles>,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
    tile_data: Res<TileData>,
    mut textures: ResMut<Assets<Texture>>,
) {
    if map_sprite_handles.atlas_loaded {
        return;
    }

    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    if let Some(LoadState::Loaded(_)) =
        asset_server.get_group_load_state(&map_sprite_handles.handles)
    {
        for texture_id in map_sprite_handles.handles.iter() {
            let handle = Handle::from_id(*texture_id);
            let texture = textures.get(&handle).unwrap();
            texture_atlas_builder.add_texture(handle, &texture);
        }

        let mut texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();

        create_map(
            &mut commands,
            &map,
            &tile_data,
            &asset_server,
            &mut texture_atlas,
        );

        map_sprite_handles.atlas_loaded = true;
    }
}
