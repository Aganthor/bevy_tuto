use bevy::{
    asset::{LoadState},
    prelude::*,
    sprite::TextureAtlasBuilder,
};

use super::map_creator::*;

pub struct MapPlugin;

#[derive(Default)]
pub struct MapSpriteHandles {
    handles: Vec<HandleUntyped>,
    pub atlas_handle: Handle<TextureAtlas>,
    atlas_loaded: bool,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MapSpriteHandles>()
            .init_resource::<Map>()
            .init_resource::<TileData>()
            .add_startup_system(load_and_prepare_sprites)
            .add_system(render_map);
    }
}

fn load_and_prepare_sprites(
    mut map_sprite_handles: ResMut<MapSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Texture>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>
) {
    map_sprite_handles.handles = asset_server.load_folder("map_tiles/").unwrap();

    let mut texture_atlas_builder = TextureAtlasBuilder::default();

    if let LoadState::Loaded =
        asset_server.get_group_load_state(map_sprite_handles.handles.iter().map(|handle| handle.id))
    {
        for texture_id in map_sprite_handles.handles.iter() {
            let texture = textures.get(texture_id).unwrap();
            texture_atlas_builder.add_texture(texture_id.clone_weak().typed::<Texture>(), &texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        map_sprite_handles.atlas_handle = texture_atlases.add(texture_atlas);
                
        map_sprite_handles.atlas_loaded = true;
    }
}
