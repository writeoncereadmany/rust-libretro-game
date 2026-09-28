use crate::app::pandamonium::AfterUpdate;
use crate::component::graphics::Sprite;
use crate::component::physics::{Position, Velocity};
use crate::entities::hero::{DirectionFacing, Hero, HeroState, MovementIntent};
use crate::game::game::Character;
use derive::system;
use engine::entities::entity::Entities;
use engine::events::event::Events;

#[system]
fn update_sprite(_update: &AfterUpdate, world: &mut Entities, _events: &mut Events) {
    world.apply(|(Hero(), status, facing, Velocity(dx, _))| match status {
        HeroState::WallDragLeft => DirectionFacing::RIGHT,
        HeroState::WallDragRight => DirectionFacing::LEFT,
        HeroState::Airborne => facing,
        _otherwise => {
            if dx > 0.0 {
                DirectionFacing::RIGHT
            } else if dx < 0.0 {
                DirectionFacing::LEFT
            } else {
                facing
            }
        }
    });
    world.apply(
        |(
             Hero(),
             status,
             facing,
             character,
             movement_intent,
             Position(x, _y),
             Velocity(dx, dy),
         )| {
            let suffix = match status {
                HeroState::Grounded => {
                    if dx == 0.0 {
                        "_stand"
                    } else {
                        if turning(&facing, &movement_intent) {
                            "_skid"
                        } else {
                            let frame = (x as i32 / 8) % 4;
                            match frame {
                                0 => "_run_1",
                                1 => "_run_2",
                                2 => "_run_3",
                                3 => "_run_2",
                                _ => "error",
                            }
                        }
                    }
                }
                HeroState::Airborne => {
                    if dy > 0.0 {
                        "_ascend"
                    } else {
                        "_descend"
                    }
                }
                HeroState::WallDragLeft => "_wallslide",
                HeroState::WallDragRight => "_wallslide",
                HeroState::Submerged => {
                    if dy < 0.0 {
                        "_swim_down"
                    } else {
                        let frame = (x as i32 / 16) % 2;
                        match frame {
                            0 => "_swim_1",
                            1 => "_swim_2",
                            _ => "error",
                        }
                    }
                }
            };
            let sprite = match character {
                Character::Bluu => "panda",
                Character::Redd => "redd",
            }
                .to_string()
                + suffix;
            Sprite::sprite_ex(sprite, 10, flip(&facing))
        },
    );
}

fn turning(facing: &DirectionFacing, movement_intent: &MovementIntent) -> bool {
    match (movement_intent, facing) {
        (MovementIntent::LEFT, DirectionFacing::RIGHT) => true,
        (MovementIntent::RIGHT, DirectionFacing::LEFT) => true,
        _otherwise => false,
    }
}

fn flip(facing: &DirectionFacing) -> bool {
    match facing {
        DirectionFacing::LEFT => true,
        DirectionFacing::RIGHT => false,
    }
}