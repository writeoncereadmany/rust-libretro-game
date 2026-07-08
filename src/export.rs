use std::env;
use std::fs::File;
use std::io::Write;
use engine::assets::Assets;

pub fn main() -> std::io::Result<()> {
    engine::export::export_assets()
}