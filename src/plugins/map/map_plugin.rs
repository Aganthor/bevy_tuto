use bevy::prelude::*;

use super::map_creator::*;

pub struct MapPlugin;

#[derive(Default)]
pub struct MapSpriteHandles {
    pub handles: Vec<HandleUntyped>,
    pub atlas_handle: Handle<TextureAtlas>,
    pub atlas_loaded: bool,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MapSpriteHandles>()
            .init_resource::<Map>()
            .init_resource::<TileData>()
            .add_startup_system(load_sprites)
            .add_system(render_map);
    }
}

fn load_sprites(
    mut map_sprite_handles: ResMut<MapSpriteHandles>,
    asset_server: Res<AssetServer>
) {
    info!("load_sprites system");
    map_sprite_handles.handles = asset_server.load_folder("map_tiles/").unwrap();
}
