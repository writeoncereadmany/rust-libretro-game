use crate::app::pandamonium::StartGame;
use crate::game::game::Character;
use crate::screens::screen::Screen;
use engine::events::event::{Event, Events};
use engine::events::input::ButtonPressed;
use engine::renderer::asset_renderer::AssetRenderer;
use engine::renderer::spritefont::Alignment;
use engine::renderer::spritefont::HorizontalAlignment::CENTER;
use engine::renderer::spritefont::VerticalAlignment::MIDDLE;
use rust_libretro::types::JoypadState;

pub struct TitleScreen {
    character: Character,
}

impl TitleScreen {
    pub fn new() -> Self {
        TitleScreen {
            character: Character::Bluu,
        }
    }
}

impl Screen for TitleScreen {
    fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|ButtonPressed(button)| {
            if button == &JoypadState::START {
                events.fire(StartGame(self.character.clone()))
            }
            if button == &JoypadState::RIGHT {
                self.character = Character::Redd;
            }            
            if button == &JoypadState::LEFT {
                self.character = Character::Bluu;
            }
        });
    }

    fn draw(&mut self, renderer: &mut AssetRenderer) {
        renderer.clear();
        renderer.draw_text(
            "Pandamonium!",
            "Spritefont_Medium",
            192,
            108,
            Alignment::aligned(CENTER, MIDDLE),
        );
        renderer.draw_text(
            match self.character {
                Character::Bluu => "< Play as Bluu  ",
                Character::Redd => "  Play as Redd >",
            },
            "Spritefont_Medium",
            192,
            48,
            Alignment::aligned(CENTER, MIDDLE),
        );
        renderer.draw_sprite("panda_stand", 100, 42, false);
        renderer.draw_sprite("redd_stand", 276, 42, true);
    }
}
