use crate::renderer::indexed_texture::IndexedTexture;
use crate::renderer::spritefont::SpriteFont;
use crate::renderer::tilesheet::TileSheet;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tar::Archive;
use tiled::PropertyValue::StringValue;
use crate::assets::map::Map;

pub struct Assets {
    pub tilesheets: HashMap<String, Arc<TileSheet>>,
    pub maps: HashMap<String, Map>,
    pub fonts: HashMap<String, SpriteFont>,
}

impl Assets {
    pub fn new() -> Self {
        Assets {
            tilesheets: HashMap::new(),
            maps: HashMap::new(),
            fonts: HashMap::new(),
        }
    }

    pub fn load_assets(&mut self, archive: &mut Archive<&[u8]>) {
        let mut textures = HashMap::new();
        let mut tilesets = Vec::new();
        let mut tilemaps = HashMap::new();

        let mut map_loader = tiled::Loader::new();

        for entry in archive.entries().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path().unwrap();
            if extension(&path, "png") {
                let filename = filename(&path);
                let decoder = png::Decoder::new(entry);
                let sheet = IndexedTexture::from_png(decoder);
                textures.insert(filename, sheet);
            } else if extension(&path, "tmx") {
                let map = map_loader.load_tmx_map(&path).unwrap();
                tilemaps.insert(filename(&path), map);
            } else if extension(&path, "tsx") {
                let tileset = map_loader.load_tsx_tileset(&path).unwrap();
                tilesets.push(tileset);
            }
        }

        for tileset in tilesets {
            let user_type = &tileset.user_type;
            let tile_filename = &filename(&tileset.image.as_ref().unwrap().source);
            let texture = textures.remove(tile_filename).unwrap();
            let tilesheet = Arc::new(TileSheet::new(
                texture.palette,
                texture.texture,
                tileset.tile_width,
                tileset.tile_height,
                tileset.columns,
            ));

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
            } else {
                self.tilesheets.insert(tileset.name.clone(), tilesheet);
            }
        }

        for (name, map) in tilemaps {
            self.maps.insert(name, Map::new(&map, &self.tilesheets));
        }
    }
}

fn filename(path: &Path) -> String {
    path.file_stem()
        .map(|filename| filename.to_string_lossy().to_string())
        .unwrap_or_else(String::new)
}

fn extension(path: &Path, extension: &str) -> bool {
    path.extension()
        .map(|ext| ext.eq_ignore_ascii_case(extension))
        .unwrap_or(false)
}