use base64::{alphabet, engine, Engine};
use base64::engine::general_purpose;
use crate::renderer::sprite::{Bounds, Sprite};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

const CUSTOM_ENGINE: engine::GeneralPurpose =
    engine::GeneralPurpose::new(&alphabet::URL_SAFE, general_purpose::NO_PAD);

#[derive(Serialize, Deserialize)]
pub struct TileSheet {
    pub name: String,
    pub palette: Vec<u16>,
    #[serde(serialize_with = "as_base64", deserialize_with = "from_base64")]
    pub tile_sheet: Vec<u8>,
    pub tile_width: u32,
    pub tile_height: u32,
    pub columns: u32
}

fn as_base64<T, S>(key: &T, serializer: S) -> Result<S::Ok, S::Error>
where T: AsRef<[u8]>,
      S: Serializer
{
    serializer.serialize_str(&CUSTOM_ENGINE.encode(key))
}

fn from_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where D: Deserializer<'de>
{
    use serde::de::Error;
    String::deserialize(deserializer)
        .and_then(|string| CUSTOM_ENGINE.decode(&string).map_err(|err| Error::custom(err.to_string())))
}

impl TileSheet {
    pub fn new(
        name: &str,
        palette: Vec<u16>,
        tile_sheet: Vec<u8>,
        tile_width: u32,
        tile_height: u32,
        columns: u32,
    ) -> Self {
        TileSheet {
            name: name.to_string(),
            palette,
            tile_sheet,
            tile_width,
            tile_height,
            columns,
        }
    }

    pub fn width(&self) -> u32 {
        self.tile_width * self.columns
    }

    pub fn sprite(self: &Self, column: u32, row: u32) -> Sprite {
        Sprite {
            tile_sheet: self.name.clone(),
            bounds: Bounds {
                x: column * self.tile_width,
                y: row * self.tile_height,
                width: self.tile_width,
                height: self.tile_height,
            },
        }
    }

    pub fn tile(self: &Self, tile_id: u32) -> Sprite {
        self.sprite(tile_id % self.columns, tile_id / self.columns)
    }
}