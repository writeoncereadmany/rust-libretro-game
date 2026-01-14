use std::time::Duration;
use crate::assets::assets::Assets;
use crate::component::graphics::Sprite;
use engine::events::event::Events;
use crate::game::game::UpdateBackgroundTile;

pub fn setup_flashlamps(events: &mut Events) {
    let mut flashlamps: Vec<(i32, i32)> = Vec::new();
    for x in 17..30 {
        flashlamps.push((x, 0))
    }
    for y in 1..19 {
        flashlamps.push((29, y))
    }
    for x in 0..30 {
        flashlamps.push((29 - x, 19))
    }
    for y in 1..19 {
        flashlamps.push((0, 19 - y))
    }
    for x in 0..12 {
        flashlamps.push((x, 0))
    }

    for (i, (x, y)) in flashlamps.iter().enumerate() {
        let (x, y) = (x * 12, y * 12);
        let fraction_of_fulltime = i as f64 / flashlamps.len() as f64;
        let fire_in = Duration::from_secs_f64(2.4 + (10.0 * fraction_of_fulltime));
        events.fire(UpdateBackgroundTile { x, y, sprite: Sprite("lamp_unlit") });
        events.schedule(fire_in, UpdateBackgroundTile{x, y, sprite: Sprite("lamp_green")});
    }
}