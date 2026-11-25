use derive::{system, Event};
use engine::entities::entity::{entity, Entities};
use engine::events::event::Events;
use engine::renderer::spritefont::Alignment;
use engine::renderer::spritefont::HorizontalAlignment::CENTER;
use engine::renderer::spritefont::VerticalAlignment::MIDDLE;
use crate::component::graphics::Sprite::Text;
use crate::component::physics::{Position, Velocity};

#[derive(Event)]
pub struct DisplayMessage(pub String);

#[system]
fn display_message(DisplayMessage(message): &DisplayMessage, world: &mut Entities, _events: &mut Events) {
    world.spawn(entity()
        .with(Text(
            message.clone(),
            "Spritefont_Medium".to_string(),
            Alignment::aligned(CENTER, MIDDLE),
            20))
        .with(Position(192.0, 92.0))
        .with(Velocity(0.0, 20.0))
    );
}
