pub mod map;

use crate::assets::map::Map;
use crate::renderer::indexed_texture::IndexedTexture;
use crate::renderer::sprite::Sprite;
use crate::renderer::spritefont::SpriteFont;
use crate::renderer::tilesheet::TileSheet;
use glob::glob;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use tiled::PropertyValue::StringValue;

#[derive(Serialize, Deserialize)]
pub struct Assets {
    pub tilesheets: HashMap<String, TileSheet>,
    pub maps: HashMap<String, Map>,
    pub fonts: HashMap<String, SpriteFont>,
    pub sprites: HashMap<String, Sprite>
}

impl Assets {
    pub fn new() -> Self {
        Assets {
            tilesheets: HashMap::new(),
            maps: HashMap::new(),
            fonts: HashMap::new(),
            sprites: HashMap::new()
        }
    }

    pub fn load_from_filesystem(&mut self, path: &str) {
        let mut textures = HashMap::new();
        let mut tilesets = Vec::new();

        let mut map_loader = tiled::Loader::new();

        for entry in glob(&(path.to_string() + "/**/*.png")).unwrap() {
            match entry {
                Ok(image) => {
                    let filename = filename(&image);
                    let file = File::open(&image).unwrap();
                    let decoder = png::Decoder::new(&file);
                    let sheet = IndexedTexture::from_png(decoder);
                    textures.insert(filename, sheet);
                },
                Err(e) => panic!("Failed to read image: {}", e),
            }
        }

        for entry in glob(&(path.to_string() + "/**/*.tmx")).unwrap() {
            match entry {
                Ok(tilemap) => {
                    let map = map_loader.load_tmx_map(&tilemap).unwrap();
                    self.maps.insert(filename(&tilemap), map::load_map(map));
                }
                Err(e) => panic!("Failed to read map: {}", e),
            }
        }

        for entry in glob(&(path.to_string() + "/**/*.tsx")).unwrap() {
            match entry {
                Ok(tileset) => {
                    let tileset = map_loader.load_tsx_tileset(&tileset).unwrap();
                    tilesets.push(tileset);
                }
                Err(e) => panic!("Failed to read map: {}", e),
            }
        }


        for tileset in tilesets {
            let user_type = &tileset.user_type;
            let tile_filename = &filename(&tileset.image.as_ref().unwrap().source);
            let texture = textures.remove(tile_filename).unwrap();
            let tilesheet = TileSheet::new(
                &tileset.name,
                texture.palette,
                texture.texture,
                tileset.tile_width,
                tileset.tile_height,
                tileset.columns,
            );

            if user_type == &Some("Font".to_string()) {
                let mut glyphs = HashMap::new();
                let mut error_glyph = Option::None;
                for (tile_id, tile) in tileset.tiles() {
                    let x = tile_id % tileset.columns;
                    let y = tile_id / tileset.columns;
                    match tile.properties.get("Glyph") {
                        Some(StringValue(glyph)) => {
                            if glyph.len() == 1 {
                                glyphs.insert(glyph.chars().nth(0).unwrap(), tilesheet.sprite(x, y));
                            }
                            else if glyph == "ERROR" {
                                error_glyph = Some(tilesheet.sprite(x, y));
                            }
                        }
                        _otherwise => {}
                    }
                }
                self.fonts.insert(
                    tileset.name.clone(),
                    SpriteFont::new(glyphs, tileset.tile_width, tileset.tile_height, error_glyph.unwrap()),
                );
                self.tilesheets.insert(tileset.name.clone(), tilesheet);
            } else if user_type == &Some("Sprite".to_string()) {
                for (tile_id, tile) in tileset.tiles() {
                    let x = tile_id % tileset.columns;
                    let y = tile_id / tileset.columns;
                    match &tile.user_type {
                        Some(name) => {
                            self.sprites.insert(name.clone(), tilesheet.sprite(x, y));
                        }
                        _otherwise => {}
                    }
                }
                self.tilesheets.insert(tileset.name.clone(), tilesheet);
            } else {
                self.tilesheets.insert(tileset.name.clone(), tilesheet);
            }
        }
    }

    pub fn sprite(&self, name: &str) -> &Sprite {
        self.sprites.get(name).unwrap_or(self.sprites.get("error").unwrap())
    }
}

fn filename(path: &Path) -> String {
    path.file_stem()
        .map(|filename| filename.to_string_lossy().to_string())
        .unwrap_or_else(String::new)
}