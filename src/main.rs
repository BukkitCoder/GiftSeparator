use image::io::Reader as ImageReader;
use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct AssetList {
    width: u32,
    height: u32,
    frames: Vec<Vec<u32>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json_file = File::open("src/assetsList.json")?;
    let asset_list: AssetList = serde_json::from_reader(BufReader::new(json_file))?;

    let img = ImageReader::open("src/gifts@1x.png")?.decode()?;
    let output_dir = Path::new("gifts_split");
    std::fs::create_dir_all(output_dir)?;

    for (i, frame) in asset_list.frames.iter().enumerate() {
        if frame.len() < 4 { continue; }

        let x = frame[0];
        let y = frame[1];
        let width = frame[2];
        let height = frame[3];

        let gift = img.crop_imm(x, y, width, height);
        gift.save(output_dir.join(format!("gift_{:02}.png", i)))?;
    }

    println!("✅ Готово! Сохранено {} подарков", asset_list.frames.len());
    Ok(())
}
