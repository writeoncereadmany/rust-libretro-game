use engine::events::event::Events;
use engine::renderer::background_renderer::UpdateHudSprite;
use std::time::Duration;

pub fn setup_flashlamps(events: &mut Events) {
    let mut flashlamps: Vec<(i32, i32)> = Vec::new();
    for x in 17..30 {
        flashlamps.push((x, 19))
    }
    for y in 1..19 {
        flashlamps.push((29, 19-y))
    }
    for x in 0..30 {
        flashlamps.push((29 - x, 0))
    }
    for y in 1..19 {
        flashlamps.push((0, y))
    }
    for x in 0..11 {
        flashlamps.push((x, 19))
    }

    for (i, (x, y)) in flashlamps.iter().enumerate() {
        let (x, y) = (x * 12, y * 12);
        let fraction_of_fulltime = i as f64 / flashlamps.len() as f64;
        let refire_amber = ((0.5 - fraction_of_fulltime) * 0.25) + 0.5;
        let refire_red = ((0.75 - fraction_of_fulltime) * 0.25) + 0.75;
        let distance_from_center = (1.0 - ((fraction_of_fulltime - 0.5).abs() * 2.0)) * 0.6;
        events.fire(UpdateHudSprite { x, y, sprite: "lamp_unlit".to_string() });

        events.schedule("Game", Duration::from_secs_f64(distance_from_center), UpdateHudSprite {x, y, sprite: "lamp_red".to_string() });
        events.schedule("Game", Duration::from_secs_f64(distance_from_center + 0.2), UpdateHudSprite {x, y, sprite: "lamp_unlit".to_string() });

        events.schedule("Game", Duration::from_secs_f64(distance_from_center + 0.8), UpdateHudSprite {x, y, sprite: "lamp_amber".to_string() });
        events.schedule("Game", Duration::from_secs_f64(distance_from_center + 1.0), UpdateHudSprite {x, y, sprite: "lamp_unlit".to_string() });

        events.schedule("Game", Duration::from_secs_f64(distance_from_center + 1.6), UpdateHudSprite {x, y, sprite: "lamp_green".to_string() });
        events.schedule("Game", Duration::from_secs_f64(distance_from_center + 1.8), UpdateHudSprite {x, y, sprite: "lamp_unlit".to_string() });

        if fraction_of_fulltime < 0.5 {
            events.schedule("Game", fire_at(fraction_of_fulltime), UpdateHudSprite {x, y, sprite: "lamp_green".to_string() });
            events.schedule("Game", fire_at(refire_amber), UpdateHudSprite {x, y, sprite: "lamp_amber".to_string() });
            events.schedule("Game", fire_at(refire_red), UpdateHudSprite {x, y, sprite: "lamp_red".to_string() });
        } else if fraction_of_fulltime < 0.75 {
            events.schedule("Game", fire_at(fraction_of_fulltime), UpdateHudSprite {x, y, sprite: "lamp_amber".to_string() });
            events.schedule("Game", fire_at(refire_red), UpdateHudSprite {x, y, sprite: "lamp_red".to_string() });
        } else {
            events.schedule("Game", fire_at(fraction_of_fulltime), UpdateHudSprite {x, y, sprite: "lamp_red".to_string() });
        }
    }
}

fn fire_at(fraction_of_fulltime: f64) -> Duration {
    Duration::from_secs_f64(2.4 + (10.0 * fraction_of_fulltime))
}