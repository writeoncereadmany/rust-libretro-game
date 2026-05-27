use CollisionType::WATER;
use crate::component::physics::{Position, Translation, Velocity};
use derive::{Constant, Event};
use engine::entities::entity::{Entities, EntityId};
use engine::entities::entity::Id;
use engine::events::dispatcher::Dispatcher;
use engine::events::event::Events;
use engine::shapes::collision::Collision;
use engine::shapes::shape::Shape;
use engine::shapes::vec2d::{Vec2d, UNIT_X, UNIT_Y};
use crate::entities::map::{overlapping, CollisionType, Tilemap};
use crate::entities::map::CollisionType::{AIR, LEDGE, WALL};

#[derive(Event)]
pub struct CheckCollisions;

#[derive(Event)]
pub struct ResolveCollisions;

#[derive(Event)]
pub struct Collided(pub EntityId, pub EntityId, pub (f64, f64));

#[derive(Event)]
pub struct Push(pub EntityId, pub (f64, f64));

#[derive(Event)]
pub struct Submerged(pub EntityId, pub bool);

#[derive(Event)]
pub struct Splash(pub f64, pub f64);

#[derive(Constant, Clone)]
pub struct Actor();

#[derive(Constant, Clone)]
pub struct Interactable();

const EPSILON: f64 = 1e-8;

pub fn register(dispatcher: &mut Dispatcher) {
    dispatcher.register(handle_collisions);
    dispatcher.register(handle_push);
}

pub fn handle_collisions(_ : &CheckCollisions, world: &mut Entities, events: &mut Events)
{
    let tile_maps: Vec<(Id, Tilemap)> = world.collect();
    let obstacles: Vec<(Id, Shape, CollisionType, Position)> = world.collect();
    let translated_obstacles: Vec<(EntityId, Shape, CollisionType)> = obstacles.iter()
        .map(|(Id(entity_id), shape, tile, Position(x, y))| (*entity_id, shape.translate(&(*x, *y)), *tile)).collect();

    handle_scenery_collisions(&tile_maps, &translated_obstacles, world, events);

    handle_water_collisions(&tile_maps, world, events);

    handle_object_interactions(world, events);

    events.fire(ResolveCollisions);
}

fn handle_scenery_collisions(tile_maps: &Vec<(Id, Tilemap)>, translated_obstacles: &Vec<(EntityId, Shape, CollisionType)>, world: &mut Entities, events: &mut Events) {
    world.apply(|(Actor(), Id(hero_id), hero_shape, hero_position @ Position(x, y), hero_translation @ Translation(tx, ty))| {
        let collidables = overlapping(&tile_maps, &hero_shape, &hero_position, &hero_translation).iter().chain(translated_obstacles.iter()).map(|item| item.clone()).collect();

        let mut mtx = tx;
        let mut mty = ty;
        let mut push_x = 0.0;
        let mut push_y = 0.0;
        let starting_shape = hero_shape.translate(&(x, y));
        while let Some((entity_id, next_collision)) = next_collision(&starting_shape, &collidables, &is_impermeable, &(mtx, mty)) {
            let (px, py) = extend(&next_collision.push);
            mtx += px;
            mty += py;
            push_x += px;
            push_y += py;
            events.fire(Collided(hero_id, entity_id, (px, py)));
        }
        events.fire(Push(hero_id, (push_x, push_y)));
        Translation(mtx, mty)
    });
}

fn handle_water_collisions(tile_maps: &Vec<(Id, Tilemap)>, world: &mut Entities, events: &mut Events) {
    world.apply(|(Actor(), Id(hero_id), hero_shape, hero_position @ Position(x, y), hero_translation @ Translation(tx, ty), Velocity(dx, dy))| {
        let zones = overlapping(&tile_maps, &hero_shape, &hero_position, &hero_translation);
        let final_hero_position = (x + tx, y + ty);
        let start_center_of_mass = Shape::circle(hero_shape.translate(&(x, y)).center_of_mass(), 0.0);
        let end_center_of_mass = Shape::circle(hero_shape.translate(&final_hero_position).center_of_mass(), 0.0);

        let was_submerged = zones.iter().any(|(_, zone_shape, zone_type)| zone_type == &WATER && start_center_of_mass.intersects(zone_shape));
        let is_submerged = zones.iter().any(|(_, zone_shape, zone_type)| zone_type == &WATER && end_center_of_mass.intersects(zone_shape));

        if !was_submerged & is_submerged {
            if let Some((_, splash_collision)) = next_collision(&start_center_of_mass, &zones, &|collision_type, _| { collision_type == &WATER}, &(tx, ty)) {
                let translation_to_splash = (tx, ty).scale(&splash_collision.dt);
                let (splash_x, splash_y) = start_center_of_mass.translate(&translation_to_splash).center_of_mass();
                events.fire(Splash(splash_x, splash_y));
            }
        }

        if was_submerged & !is_submerged {
            if let Some((_, splash_collision)) = next_collision(&start_center_of_mass, &zones, &|collision_type, _| { collision_type == &AIR}, &(tx, ty)) {
                if dy < 200.0 {
                    let splash_push_extended = extend(&splash_collision.push);
                    let (sx, sy) = (tx, ty).plus(&splash_push_extended);
                    events.fire(Submerged(hero_id, true));
                    return (Velocity(dx, 0.0), Translation(sx, sy));
                }
                else {
                    let translation_to_splash@(_, sy) = (tx, ty).scale(&splash_collision.dt);
                    let (splash_x, splash_y) = start_center_of_mass.translate(&translation_to_splash).center_of_mass();
                    events.fire(Splash(splash_x, splash_y));
                }
            }
        }

        events.fire(Submerged(hero_id, is_submerged));

        (Velocity(dx, dy), Translation(tx, ty))
    });
}

fn handle_object_interactions(world: &mut Entities, events: &mut Events) {
    world.for_each_pair(|(Actor(), Id(actor_id), hero_shape, hero_position, hero_translation),
                         (Interactable(), Id(interactable_id), pickup_shape, pickup_position, pickup_translation)|
        {
            if actor_id != interactable_id {
                if let Some(collision) = collides(hero_shape, hero_position, hero_translation, pickup_shape, pickup_position, pickup_translation)
                {
                    events.fire(Collided(*actor_id, *interactable_id, collision.push));
                }
            }
        }
    );
}

fn extend(val: &(f64, f64) ) -> (f64, f64) {
    val.plus(&val.unit().scale(&EPSILON))
}

fn collides(moving: &Shape, &Position(mx, my): &Position, &Translation(mtx, mty): &Translation,
            other: &Shape, &Position(ox, oy): &Position, other_translation: &Option<Translation>) -> Option<Collision>
{
    let moving = moving.translate(&(mx, my));
    let other = other.translate(&(ox, oy));
    if let Some(Translation(otx, oty)) = other_translation {
        moving.collides(&other, &(mtx - otx, mty - oty))
    }
    else {  
        moving.collides(&other, &(mtx, mty))
    }
}

fn next_collision(shape: &Shape, collidables: &Vec<(EntityId, Shape, CollisionType)>, condition: &dyn Fn(&CollisionType, &Collision) -> bool, translation: &(f64, f64)) -> Option<(EntityId, Collision)> {
    let mut collisions: Vec<(EntityId, Collision)> = collidables.iter()
        .map(|(id, collidable, tile)| {
            if let Some(collision) = shape.collides(collidable, translation) {
                if condition(tile, &collision) {
                    Some((*id, collision))
                }
                else {
                    None
                }
            } else {
                None
            }
        })
        .flatten()
        .collect();

    collisions.sort_unstable_by(|(_, c1), (_, c2)| c1.dt.total_cmp(&c2.dt).reverse());
    collisions.pop()
}

fn is_impermeable(tile: &CollisionType, collision: &Collision) -> bool {
    tile == &WALL || (tile == &LEDGE && collision.push.dot(&UNIT_Y) > 0.0)
}

fn is_water(tile: &CollisionType, collision: &Collision) -> bool {
    tile == &WALL || (tile == &LEDGE && collision.push.dot(&UNIT_Y) > 0.0)
}

fn handle_push(Push(entity_id, (px, py)): &Push, world: &mut Entities, _events: &mut Events) {
    world.apply_to(entity_id, |Velocity(dx, dy)| Velocity(limit(&dx, px), limit(&dy, py)));
}

fn limit (velocity: &f64, push: &f64) -> f64 {
    if (velocity < &0.0 && push > &0.0) || (velocity > &0.0 && push < &0.0) {
        0.0
    } else {
        *velocity
    }
}