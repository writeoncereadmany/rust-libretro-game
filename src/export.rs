use std::env;
use std::fs::File;
use std::io::Write;
use engine::assets::Assets;

pub fn main() -> std::io::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    let mut file_output = File::create(args.get(2).expect("No output file specified"))?;

    let mut assets = Assets::new();
    assets.load_from_filesystem("assets");

    let binary_data = serde_json::to_vec(&assets)?;
    file_output.write(binary_data.as_slice())?;
    file_output.flush()
}