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
use crate::entities::map::CollisionType::LEDGE;

#[derive(Event)]
pub struct CheckCollisions;

#[derive(Event)]
pub struct ResolveCollisions;

#[derive(Event)]
pub struct Collided(pub EntityId, pub EntityId);

#[derive(Event)]
pub struct Push(pub EntityId, pub (f64, f64));

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

    world.apply(|(Actor(), Id(hero_id), hero_shape, hero_position@Position(x, y), hero_translation@Translation(tx, ty))| {
        let collidables = overlapping(&tile_maps, &hero_shape, &hero_position, &hero_translation).iter().chain(translated_obstacles.iter()).map(|item| item.clone()).collect();

        let mut mtx = tx;
        let mut mty = ty;
        let mut push_x = 0.0;
        let mut push_y = 0.0;
        let starting_shape = hero_shape.translate(&(x, y));
        while let Some((entity_id, next_collision)) = next_collision(&starting_shape, &collidables, &(mtx, mty)) {
            let (px, py) = extend(&next_collision.push);
            mtx += px;
            mty += py;
            push_x += px;
            push_y += py;
           events.fire(Collided(hero_id, entity_id));
        }
        events.fire(Push(hero_id, (push_x, push_y)));
        Translation(mtx, mty)
    });

    world.for_each_pair(|(Actor(), Id(actor_id), hero_shape, hero_position, hero_translation),
                         (Interactable(), Id(interactable_id), pickup_shape, pickup_position, pickup_translation)|
        {
            if actor_id != interactable_id {
                if collides(hero_shape, hero_position, hero_translation, pickup_shape, pickup_position, pickup_translation)
                {
                    events.fire(Collided(*actor_id, *interactable_id));
                }
            }
        }
    );

    events.fire(ResolveCollisions);
}

fn extend(val: &(f64, f64) ) -> (f64, f64) {
    val.plus(&val.unit().scale(&EPSILON))
}

fn collides(moving: &Shape, &Position(mx, my): &Position, &Translation(mtx, mty): &Translation,
            other: &Shape, &Position(ox, oy): &Position, other_translation: &Option<Translation>) -> bool
{
    let moving = moving.translate(&(mx, my));
    let other = other.translate(&(ox, oy));
    if let Some(Translation(otx, oty)) = other_translation {
        moving.intersects_moving(&other, &(mtx - otx, mty - oty))
    }
    else {  
        moving.intersects_moving(&other, &(mtx, mty))
    }
}

fn next_collision(shape: &Shape, collidables: &Vec<(EntityId, Shape, CollisionType)>, translation: &(f64, f64)) -> Option<(EntityId, Collision)> {
    let mut collisions: Vec<(EntityId, Collision)> = collidables.iter()
        .map(|(id, collidable, tile)| {
            if let Some(collision) = shape.collides(collidable, translation) {
                if tile == &LEDGE && (collision.push.dot(&UNIT_X).abs() > 1e-6 || collision.push.dot(&UNIT_Y) < 0.0)  {
                    None
                }
                else {
                    Some((*id, collision))
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