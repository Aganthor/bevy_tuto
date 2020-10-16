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
        app
            .init_resource::<MapSpriteHandles>()
            .init_resource::<Map>()
            .init_resource::<TileData>()
            .add_startup_system(setup.system())
            .add_system(load_atlas.system());
    }
}

fn setup(mut map_sprite_handles: ResMut<MapSpriteHandles>, asset_server: Res<AssetServer>) {
    map_sprite_handles.handles = asset_server
        .load_asset_folder("assets/map_tiles")
        .unwrap();
}

fn load_atlas(
    mut commands: Commands,
    mut map_sprite_handles: ResMut<MapSpriteHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if map_sprite_handles.atlas_loaded {
        return;
    }

    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    if let Some(LoadState::Loaded(_)) =
        asset_server.get_group_load_state(&map_sprite_handles.handles) {
            for texture_id in map_sprite_handles.handles.iter() {
                let handle = Handle::from_id(*texture_id);
                let texture = textures.get(&handle).unwrap();
                texture_atlas_builder.add_texture(handle, &texture);
            }

            let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
            let texture_atlas_texture = texture_atlas.texture;
            let _atlas_handle = texture_atlases.add(texture_atlas);

            // Just a quick test to see if they loaded correctly...
            commands
                .spawn(SpriteComponents {
                    material: materials.add(texture_atlas_texture.into()),
                    transform: Transform::from_translation(Vec3::new(0.0, -150.0, 1.0)),
                    ..Default::default()
                });

            map_sprite_handles.atlas_loaded = true;
        }
}