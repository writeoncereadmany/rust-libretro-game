use crate::component::graphics::Sprite;
use crate::game::game::{UpdateBackgroundText, UpdateBackgroundTile};
use engine::events::event::Events;
use engine::renderer::spritefont::Alignment;
use engine::renderer::spritefont::HorizontalAlignment::RIGHT;
use engine::renderer::spritefont::VerticalAlignment::MIDDLE;

pub fn setup_hud(events: &mut Events) {
    update_bonus(1, events);
    update_score(12345678, events);
}

pub fn update_bonus(bonus: u32, events: &mut Events) {
    let sprite = Sprite(match bonus {
        1 => "bonus_1",
        2 => "bonus_2",
        3 => "bonus_3",
        4 => "bonus_4",
        5 => "bonus_5",
        _otherwise => "error"
    });

    events.fire(UpdateBackgroundTile { x: 12*12, y: 0, sprite})
}

pub fn update_score(score: u32, events: &mut Events) {
    events.fire(UpdateBackgroundTile { x: 13*12, y: 0, sprite: Sprite("score_left") });
    events.fire(UpdateBackgroundTile { x: 14*12, y: 0, sprite: Sprite("score_mid") });
    events.fire(UpdateBackgroundTile { x: 15*12, y: 0, sprite: Sprite("score_mid") });
    events.fire(UpdateBackgroundTile { x: 16*12, y: 0, sprite: Sprite("score_right") });

    events.fire(UpdateBackgroundText {
        x: 17*12 - 3,
        y: 6,
        font: "Spritefont_Medium",
        text: simplify(score),
        alignment: Alignment::aligned(RIGHT, MIDDLE)});
}

fn simplify(score: u32) -> String {
    if (score <= 99_999) {
        score.to_string()
    } else if (score <= 9_999_999) {
        (score / 1_000).to_string() + "k"
    } else if (score <= 99_999_999) {
        format!("{:.1}m", score as f64 / 1_000_000.0)
    } else {
        (score / 1_000_000).to_string() + "m"
    }
}