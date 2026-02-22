use engine::events::event::Events;
use engine::renderer::background_renderer::{UpdateBackgroundSprite, UpdateBackgroundText};
use engine::renderer::spritefont::Alignment;
use engine::renderer::spritefont::HorizontalAlignment::RIGHT;
use engine::renderer::spritefont::VerticalAlignment::MIDDLE;

pub fn setup_hud(events: &mut Events, score: u32, bonus: u32) {
    update_bonus(bonus, events);
    update_score(score, events);
}

pub fn update_bonus(bonus: u32, events: &mut Events) {
    let sprite = match bonus {
        1 => "bonus_1",
        2 => "bonus_2",
        3 => "bonus_3",
        4 => "bonus_4",
        5 => "bonus_5",
        _otherwise => "error"
    }.to_string();

    events.fire(UpdateBackgroundSprite { x: 12*12, y: 19*12, sprite})
}

pub fn update_score(score: u32, events: &mut Events) {
    events.fire(UpdateBackgroundSprite { x: 13*12, y: 19*12, sprite: "score_left".to_string() });
    events.fire(UpdateBackgroundSprite { x: 14*12, y: 19*12, sprite: "score_mid".to_string() });
    events.fire(UpdateBackgroundSprite { x: 15*12, y: 19*12, sprite: "score_mid".to_string() });
    events.fire(UpdateBackgroundSprite { x: 16*12, y: 19*12, sprite: "score_right".to_string() });

    events.fire(UpdateBackgroundText {
        x: 17*12 - 3,
        y: 19*12 + 6,
        font: "Spritefont_Medium",
        text: simplify(score),
        alignment: Alignment::aligned(RIGHT, MIDDLE)});
}

fn simplify(score: u32) -> String {
    if score <= 99_999 {
        score.to_string()
    } else if score <= 9_999_999 {
        (score / 1_000).to_string() + "k"
    } else if score <= 99_999_999 {
        format!("{:.1}m", score as f64 / 1_000_000.0)
    } else {
        (score / 1_000_000).to_string() + "m"
    }
}