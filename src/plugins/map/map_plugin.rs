use bevy::{
    asset::LoadState,
    prelude::*, 
    utils::HashSet,
    sprite::{TextureAtlas, TextureAtlasBuilder},
};
use bevy_tilemap::prelude::*;

use super::map_creator::*;

const CHUNK_WIDTH: u32 = 16;
const CHUNK_HEIGHT: u32 = 16;
const TILEMAP_WIDTH: i32 = CHUNK_WIDTH as i32 * 40;
const TILEMAP_HEIGHT: i32 = CHUNK_HEIGHT as i32 * 40;

pub struct MapPlugin;

#[derive(Default, Clone)]
pub struct TileSpriteHandles {
    pub handles: Vec<HandleUntyped>,
    pub atlas_loaded: bool,
}

#[derive(Default, Clone)]
pub struct MapState {
    pub map_loaded: bool,
    pub spawned: bool,
    pub collisions: HashSet<(i32, i32)>,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<TileSpriteHandles>()
            .init_resource::<Map>()
            .init_resource::<TileData>()
            .add_plugins(TilemapDefaultPlugins)
            .add_startup_system(setup.system())
            .add_system(load.system())
            .add_system(generate_random_world.system());
    }
}

fn setup(
    mut tile_sprite_handles: ResMut<TileSpriteHandles>,
    asset_server: Res<AssetServer>
) {
    tile_sprite_handles.handles = asset_server.load_folder("assets/map_tiles").unwrap();
}

fn load(
    mut commands: Commands,
    mut sprite_handles: ResMut<TileSpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    asset_server: Res<AssetServer>,
) {
    if sprite_handles.atlas_loaded {
        return;
    }

    // Let's load our tile textures.
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    if let LoadState::Loaded = asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        for handle in sprite_handles.handles.iter() {
            let texture = textures.get(handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), &texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let atlas_handle = texture_atlases.add(texture_atlas);

        let tilemap = Tilemap::builder()
            .dimensions(TILEMAP_WIDTH as u32, TILEMAP_HEIGHT as u32)
            .chunk_dimensions(CHUNK_WIDTH, CHUNK_HEIGHT, 1)
            .texture_dimensions(32, 32)
            .auto_chunk()
            .auto_spawn(2, 2)
            .add_layer(TilemapLayer {
                kind: LayerKind::Dense,
                ..Default::default()
            },
            0,
            )
            .texture_atlas(atlas_handle)
            .finish()
            .unwrap();

        let tilemap_components = TilemapBundle {
            tilemap,
            visible: Visible {
                is_visible: true,
                is_transparent: true,
            },
            transform: Default::default(),
            global_transform: Default::default(),
        };
        commands
            .spawn()
            .insert_bundle(tilemap_components)
            .insert(Timer::from_seconds(0.075, true));

        sprite_handles.atlas_loaded = true;
    }
}