use crate::entities::entity::Entities;
use crate::events::dispatcher::Dispatcher;
use crate::events::event::{EventTrait, Events};
use crate::renderer::spritefont::Alignment;
use std::any::Any;
use tiled::TileId;

pub struct UpdateBackgroundSprite {
    pub x: i32,
    pub y: i32,
    pub sprite: String,
}

impl EventTrait for UpdateBackgroundSprite {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events) {
        dispatcher.dispatch(self, world, events);
    }
}

pub struct UpdateHudSprite {
    pub x: i32,
    pub y: i32,
    pub sprite: String,
}

impl EventTrait for UpdateHudSprite {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events) {
        dispatcher.dispatch(self, world, events);
    }
}

pub struct UpdateBackgroundTile {
    pub x: i32,
    pub y: i32,
    pub tileset: String,
    pub tile: TileId
}

impl EventTrait for UpdateBackgroundTile {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events) {
        dispatcher.dispatch(self, world, events);
    }
}

pub struct UpdateBackgroundText {
    pub x: i32,
    pub y: i32,
    pub font: &'static str,
    pub text: String,
    pub alignment: Alignment
}

impl EventTrait for UpdateBackgroundText {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events) {
        dispatcher.dispatch(self, world, events);
    }
}
pub struct UpdateHudText {
    pub x: i32,
    pub y: i32,
    pub font: &'static str,
    pub text: String,
    pub alignment: Alignment
}

impl EventTrait for UpdateHudText {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events) {
        dispatcher.dispatch(self, world, events);
    }
}