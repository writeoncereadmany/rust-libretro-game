use derive::system;
use engine::entities::entity::Entities;
use engine::events::event::Events;
use crate::component::collisions::{Push, Submerged};
use crate::entities::hero::{Hero, HeroState};

#[system]
fn on_push(Push(entity_id, (px, py)): &Push, world: &mut Entities, _events: &mut Events) {
    world.apply_to(entity_id, |Hero()| {
        if py > &0.0 {
            HeroState::Grounded
        } else if px < &0.0 {
            HeroState::WallDragRight
        } else if px > &0.0 {
            HeroState::WallDragLeft
        } else {
            HeroState::Airborne
        }
    });
}

#[system]
fn on_submerged(
    Submerged(entity_id, submerged): &Submerged,
    world: &mut Entities,
    _events: &mut Events,
) {
    world.apply_to(entity_id, |(Hero(), hero_state)| {
        if *submerged {
            HeroState::Submerged
        } else {
            hero_state
        }
    });
}
