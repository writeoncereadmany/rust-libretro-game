use crate::assets::Assets;
use crate::events::event::{Event, Events};
use crate::renderer::background_renderer::{UpdateBackgroundSprite, UpdateBackgroundText, UpdateBackgroundTile, UpdateHudSprite, UpdateHudText};
use crate::renderer::renderer::Renderer;
use crate::renderer::spritefont::Alignment;
use rust_libretro::contexts::RunContext;
use std::sync::Arc;
use tiled::TileId;

pub struct AssetRenderer {
    renderer: Renderer,
    assets: Arc<Assets>,
}

impl AssetRenderer {
    pub fn new(renderer: Renderer, assets: Arc<Assets>) -> Self {
        AssetRenderer { renderer, assets }
    }

    pub fn draw_background(&mut self, tileset: &str, tile: TileId, x: i32, y: i32) {
        if let Some(tilesheet) = self.assets.tilesheets.get(tileset) {
            self.renderer.draw_background(&tilesheet.tile(tile), x, y);
        }
        else {
            self.draw_background_sprite("error", x, y);
        }
    }

    pub fn draw_background_sprite(&mut self, sprite: &str, x: i32, y: i32) {
        self.renderer.draw_background(self.assets.sprite(sprite), x, y);
    }
    
    pub fn draw_hud_sprite(&mut self, sprite: &str, x: i32, y: i32) {
        self.renderer.draw_hud(self.assets.sprite(sprite), x, y);
    }

    pub fn draw_sprite(&mut self, sprite: &str, x: i32, y: i32, flip_x: bool) {
        self.renderer.draw_sprite(self.assets.sprite(sprite), x, y, flip_x);
    }

    pub fn draw_text(&mut self, text: &str, font: &str, x: i32, y: i32, alignment: Alignment) {
        self.renderer.draw_text(self.assets.fonts.get(font).unwrap(), text, x, y, alignment);
    }

    pub fn draw_background_text(&mut self, text: &str, font: &str, x: i32, y: i32, alignment: Alignment) {
        self.renderer.draw_background_text(self.assets.fonts.get(font).unwrap(), &text, x, y, alignment);
    }
    
    pub fn draw_hud_text(&mut self, text: &str, font: &str, x: i32, y: i32, alignment: Alignment) {
        self.renderer.draw_hud_text(self.assets.fonts.get(font).unwrap(), &text, x, y, alignment);
    }
    
    pub fn render(&mut self, ctx: &mut RunContext) {
        self.renderer.render_hud();
        self.renderer.render(ctx);
    }

    pub fn clear(&mut self) {
        self.renderer.clear();
    }

    pub fn clear_sprites(&mut self) {
        self.renderer.clear_sprites();
    }

    pub fn on_event(&mut self, event: &Event, _events: &mut Events) {

        event.apply(|UpdateBackgroundSprite { x, y, sprite }| {
            self.draw_background_sprite(sprite, *x, *y)
        });

        event.apply(|UpdateBackgroundTile { x, y, tileset, tile }| {
            self.draw_background(&tileset, *tile, *x, *y)
        });

        event.apply(|UpdateBackgroundText { x, y, font, text, alignment}|{
            self.draw_background_text(&text, font, *x, *y, *alignment)
        });
        
        event.apply(|UpdateHudSprite { x, y, sprite }| {
            self.draw_hud_sprite(sprite, *x, *y)
        });

        event.apply(|UpdateHudText { x, y, font, text, alignment}|{
            self.draw_hud_text(&text, font, *x, *y, *alignment)
        });
    }
}
