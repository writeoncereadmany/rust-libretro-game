use crate::component::physics::{Position, Translation};
use derive::{Constant, Event};
use engine::assets::map::Map;
use engine::entities::entity::{Entities, EntityId, Id, entity};
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::events::spawner::Spawner;
use engine::renderer::background_renderer::UpdateBackgroundTile;
use engine::shapes::projection::{Projection, Projects};
use engine::shapes::shape::Shape;
use engine::shapes::vec2d::{UNIT_X, UNIT_Y};
use std::collections::HashMap;

#[derive(Constant, Clone, Copy, PartialEq, Eq, Debug)]
pub enum CollisionType {
    WALL,
    LEDGE,
    WATER,
    AIR,
}

#[derive(Constant, Clone)]
pub struct Tilemap(i32, i32, HashMap<(i32, i32), CollisionType>);

#[derive(Event, Clone)]
struct SpawnTilemap(Tilemap);

pub fn register(dispatcher: &mut Dispatcher, _spawner: &mut Spawner) {
    dispatcher.register(spawn_map);
}

pub fn load_map(map: &Map, spawner: &Spawner, events: &mut Events) {
    let mut tile_map: HashMap<(i32, i32), CollisionType> = HashMap::new();

    for tile in &map.tiles {
        events.fire(UpdateBackgroundTile {
            x: (tile.x + 1) * 12,
            y: (18 - tile.y) * 12,
            tileset: tile.tile_set_name.clone(),
            tile: tile.id,
        });
        if let Some(user_type) = &tile.user_type {
            match user_type.as_str() {
                "Wall" => {
                    tile_map.insert((tile.x, -tile.y - 1), CollisionType::WALL);
                }
                "Ledge" => {
                    tile_map.insert((tile.x, -tile.y - 1), CollisionType::LEDGE);
                }
                "Water" => {
                    tile_map.insert((tile.x, -tile.y - 1), CollisionType::WATER);
                }
                _otherwise => {}
            };
        }
    }
    
    events.fire(SpawnTilemap(Tilemap(12, 12, tile_map)));

    for object in &map.objects {
        spawner.spawn(&object, events);
    }
}

fn spawn_map(SpawnTilemap(tilemap): &SpawnTilemap, world: &mut Entities, _events: &mut Events) {
    world.spawn(entity().with(tilemap.clone()));
}

pub fn overlapping(
    tile_maps: &Vec<(Id, Tilemap)>,
    shape: &Shape,
    position: &Position,
    translation: &Translation,
) -> Vec<(EntityId, Shape, CollisionType)> {
    tile_maps
        .iter()
        .map(|(Id(entity_id), tilemap)| {
            overlapping_map(*entity_id, tilemap, shape, position, translation)
        })
        .flatten()
        .collect()
}

pub fn overlapping_map(
    entity_id: EntityId,
    Tilemap(width, height, tilemap): &Tilemap,
    shape: &Shape,
    &Position(x, y): &Position,
    &Translation(tx, ty): &Translation,
) -> Vec<(EntityId, Shape, CollisionType)> {
    let mut overlapping = Vec::new();

    let translated_shape = shape.translate(&(x, y));
    let Projection {
        min: min_x,
        max: max_x,
    } = translated_shape.project_moving(&(tx, ty), &UNIT_X);
    let Projection {
        min: min_y,
        max: max_y,
    } = translated_shape.project_moving(&(tx, ty), &UNIT_Y);

    let min_tile_x = (min_x / *width as f64).floor() as i32;
    let max_tile_x = (max_x / *width as f64).floor() as i32;
    let min_tile_y = (min_y / *height as f64).floor() as i32;
    let max_tile_y = (max_y / *height as f64).floor() as i32;

    for x in min_tile_x..=max_tile_x {
        for y in min_tile_y..=max_tile_y {
            let tile = tilemap.get(&(x, y)).unwrap_or(&CollisionType::AIR);
            overlapping.push((
                entity_id,
                Shape::bbox(
                    (x * width) as f64,
                    (y * height) as f64,
                    *width as f64,
                    *height as f64,
                ),
                *tile,
            ))
        }
    }

    overlapping
}
