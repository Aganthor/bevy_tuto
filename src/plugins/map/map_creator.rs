use bevy::prelude::*;
use bmp::Image;
use rand::Rng;
use simdnoise::*;
use std::collections::HashMap;

pub const TILE_SIZE: u32 = 32;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TileType {
    DeepWater,
    Dirt,
    Grass,
    Forest,
    Rock,
    Sand,
    Savannah,
    ShallowWater,
    Shore,
    Snow,
    Mountain,
}
//#[derive(Copy, Clone)]
pub struct TileData {
    pub tile_data: HashMap<TileType, String>,
}

impl Default for TileData {
    fn default() -> Self {
        let mut tile_data_map: HashMap<TileType, String> = HashMap::new();
        tile_data_map.insert(
            TileType::DeepWater,
            "assets/map_tiles/deep_water.png".to_string(),
        );
        tile_data_map.insert(TileType::Dirt, "assets/map_tiles/dirt.png".to_string());
        tile_data_map.insert(TileType::Grass, "assets/map_tiles/grass.png".to_string());
        tile_data_map.insert(TileType::Forest, "assets/map_tiles/forest.png".to_string());
        tile_data_map.insert(TileType::Rock, "assets/map_tiles/rock.png".to_string());
        tile_data_map.insert(TileType::Sand, "assets/map_tiles/sand.png".to_string());
        tile_data_map.insert(
            TileType::Savannah,
            "assets/map_tiles/savannah.png".to_string(),
        );
        tile_data_map.insert(
            TileType::ShallowWater,
            "assets/map_tiles/shallow_water.png".to_string(),
        );
        tile_data_map.insert(TileType::Shore, "assets/map_tiles/shore.png".to_string());
        tile_data_map.insert(TileType::Snow, "assets/map_tiles/snow.png".to_string());
        tile_data_map.insert(
            TileType::Mountain,
            "assets/map_tiles/mountain.png".to_string(),
        );
        TileData {
            tile_data: tile_data_map,
        }
    }
}

impl TileData {
    fn get_path(&self, tile_type: TileType) -> String {
        self.tile_data[&tile_type].clone()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TileInfo {
    pub x: usize,
    pub y: usize,
    pub tile_type: TileType,
    pub explored: bool,
    pub block_view: bool,
    pub walkable: bool,
}

impl TileInfo {
    pub fn new(x: usize, y: usize, tile_type: TileType) -> TileInfo {
        TileInfo {
            x: x,
            y: y,
            tile_type: tile_type,
            explored: false,
            block_view: false,
            walkable: match tile_type {
                TileType::DeepWater
                | TileType::Rock
                | TileType::ShallowWater
                | TileType::Mountain => false,
                TileType::Dirt
                | TileType::Grass
                | TileType::Forest
                | TileType::Sand
                | TileType::Savannah
                | TileType::Shore
                | TileType::Snow => true,
            },
        }
    }
}

pub struct MapBuilder {
    seed: i32,
    frequency: f32,
    lacunarity: f32,
    gain: f32,
    octaves: u8,
    map_size: usize,
}

impl MapBuilder {
    pub fn new() -> MapBuilder {
        MapBuilder {
            seed: 0,
            frequency: 0.0,
            lacunarity: 0.0,
            gain: 0.0,
            octaves: 0,
            map_size: 0,
        }
    }

    pub fn with_seed(mut self, seed: i32) -> MapBuilder {
        self.seed = seed;
        self
    }

    pub fn with_frequency(mut self, freq: f32) -> MapBuilder {
        self.frequency = freq;
        self
    }

    pub fn with_lacunarity(mut self, lacunarity: f32) -> MapBuilder {
        self.lacunarity = lacunarity;
        self
    }

    pub fn with_gain(mut self, gain: f32) -> MapBuilder {
        self.gain = gain;
        self
    }

    pub fn with_octaves(mut self, octaves: u8) -> MapBuilder {
        self.octaves = octaves;
        self
    }

    pub fn with_size(mut self, size: usize) -> MapBuilder {
        self.map_size = size;
        self
    }

    pub fn build(&self) -> Map {
        Map {
            noise_vector: Vec::new(),
            noise_seed: self.seed,
            noise_frequency: self.frequency,
            noise_lacunarity: self.lacunarity,
            noise_gain: self.gain,
            noise_octaves: self.octaves,
            map_size: self.map_size,
            level_data: Vec::new(),
        }
    }
}

pub struct Map {
    noise_vector: Vec<f32>,
    noise_seed: i32,
    noise_frequency: f32,
    noise_lacunarity: f32,
    noise_gain: f32,
    noise_octaves: u8,
    pub map_size: usize,
    level_data: Vec<TileInfo>,
}

impl Default for Map {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        let seed = rng.gen();

        println!("Map seed is {}.", seed);

        let mut map = MapBuilder::new()
            .with_seed(seed)
            .with_frequency(0.03)
            .with_gain(2.5)
            .with_lacunarity(0.55)
            .with_octaves(2)
            .with_size(10)
            .build();

        map.generate_noise_map();
        map.generate_level();

        map
    }
}

impl Map {
    pub fn generate_noise_map(&mut self) {
        self.noise_vector = NoiseBuilder::fbm_2d(self.map_size, self.map_size)
            .with_seed(self.noise_seed)
            .with_freq(self.noise_frequency)
            .with_lacunarity(self.noise_lacunarity)
            .with_gain(self.noise_gain)
            .with_octaves(self.noise_octaves)
            .generate_scaled(0.0, 1.0);
    }
    /*
        pub fn map_max_size(&self) -> f32 {
            self.map_size as f32 * TILE_SIZE as f32
        }
    */
    pub fn generate_level(&mut self) {
        for y in 0..self.map_size {
            for x in 0..self.map_size {
                let map_value = self.noise_vector[y * self.map_size + x];
                let tile_x_pos = x * TILE_SIZE as usize;
                let tile_y_pos = y * TILE_SIZE as usize;
                let tile_type = self.biome(map_value);
                self.level_data
                    .push(TileInfo::new(tile_x_pos, tile_y_pos, tile_type));
            }
        }
    }

    fn biome(&self, map_elevation: f32) -> TileType {
        if map_elevation < 0.1 {
            return TileType::DeepWater;
        } else if map_elevation < 0.2 {
            return TileType::Shore;
        } else if map_elevation < 0.3 {
            return TileType::Grass;
        } else if map_elevation < 0.5 {
            return TileType::Forest;
        } else if map_elevation < 0.8 {
            return TileType::Savannah;
        } else if map_elevation < 0.9 {
            return TileType::Sand;
        } else if map_elevation < 0.95 {
            return TileType::Rock;
        } else {
            return TileType::Mountain;
        }
    }

    pub fn get_tileinfo_at(&self, x: usize, y: usize) -> TileInfo {
        self.level_data[y * self.map_size + x as usize]
    }

    #[allow(dead_code)]
    pub fn save_image(self) {
        let mut img = Image::new(self.map_size as u32, self.map_size as u32);

        for x in 0..self.map_size - 1 {
            for y in 0..self.map_size - 1 {
                let height = self.noise_vector[x * self.map_size + y];
                let color = 256.0 * height;
                img.set_pixel(
                    x as u32,
                    y as u32,
                    bmp::Pixel::new(color as u8, color as u8, color as u8),
                );
            }
        }
        let _ = img.save("map.bmp");
    }
}

pub fn create_map(
    commands: &mut Commands,
    map: &Res<Map>,
    tile_data: &Res<TileData>,
    asset_server: &Res<AssetServer>,
    texture_atlas: &mut TextureAtlas,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    for y in 0..10 as usize {
        for x in 0..10 as usize {
            let tile_info = map.get_tileinfo_at(x, y);
            let transform = Transform::from_translation(Vec3::new(
                x as f32 * TILE_SIZE as f32,
                y as f32 * TILE_SIZE as f32,
                5.0,
            ));
            let handle: Handle<Texture> = asset_server
                .get_handle(tile_data.get_path(tile_info.tile_type))
                .unwrap();
            let index = texture_atlas.get_texture_index(handle).unwrap();

            commands
                //.spawn(Camera2dComponents::default())
                .spawn(SpriteSheetComponents {
                    transform: transform,
                    sprite: TextureAtlasSprite::new(index as u32),
                    //texture_atlas: texture_atlas.texture,
                    ..Default::default()
                });
        }
    }
}
