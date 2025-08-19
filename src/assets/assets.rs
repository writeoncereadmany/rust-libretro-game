use crate::renderer::indexed_texture::IndexedTexture;
use crate::renderer::spritefont::SpriteFont;
use crate::renderer::tilesheet::TileSheet;
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Error;
use std::path::Path;
use tar::Archive;
use tiled::Map;

pub struct Assets {
    tilesheets: HashMap<String, TileSheet>,
    maps: HashMap<String, Map>,
    fonts: HashMap<String, SpriteFont>,
}

pub fn load_assets(archive: &mut Archive<&[u8]>) -> Result<Assets, Error> {
    let mut textures = HashMap::new();
    let mut tilesheets = HashMap::new();
    let mut tilesets = Vec::new();
    let mut fonts = HashMap::new();
    let mut maps = HashMap::new();

    let mut map_loader = tiled::Loader::new();

    for entry in archive.entries()? {
        let entry = entry?;
        let path = entry.path()?;
        if extension(&path, "png") {
            let filename = filename(&path);
            let decoder = png::Decoder::new(entry);
            let sheet = IndexedTexture::from_png(decoder);
            textures.insert(filename, sheet);
        } else if extension(&path, "tmx") {
            let map = map_loader.load_tmx_map(&path).unwrap();
            maps.insert(filename(&path), map);
        } else if extension(&path, "tsx") {
            let tileset = map_loader.load_tsx_tileset(&path).unwrap();
            tilesets.push(tileset);
        }
    }

    for (tileset) in tilesets {
        let texture = textures.remove(&filename(&tileset.source)).unwrap();
        let tilesheet = TileSheet::new(texture.palette, texture.texture, tileset.tile_width, tileset.tile_height, tileset.columns);
        tilesheets.insert(tileset.name.clone(), tilesheet);
    }
    Ok(Assets {
        tilesheets,
        maps,
        fonts,
    })
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
