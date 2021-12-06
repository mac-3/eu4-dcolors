extern crate encoding;

use encoding::{all::ISO_8859_1, Encoding};

use std::path::{Path, PathBuf};

use country_tags::CountryTags;
use utils::{color_distance, gen_colors_set};

mod country_tags;
mod utils;

fn main() {
    let code = main_inner().map(|_| 0).unwrap_or_else(|e| {
        eprintln!("{}", e);
        101
    });
    std::process::exit(code)
}

fn main_inner() -> Result<(), Box<dyn std::error::Error>> {
    let game_dir: PathBuf = std::env::args()
        .nth(1)
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Error: the game dir path is required as first argument",
            )
        })?
        .into();
    let output_dir: PathBuf = std::env::args()
        .nth(2)
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Error: the output dir path is required as second argument",
            )
        })?
        .into();
    let tags = CountryTags::parse_files(get_country_tags_files(&game_dir)?, &game_dir)?;
    let mut colors = gen_colors_set(tags.list.len());

    let dirs = ["history", "decisions", "events"]
        .into_iter()
        .map(|s| game_dir.join(s))
        .collect::<Vec<PathBuf>>();
    let list = tags.process_priority_queue(dirs)?;

    for c in list {
        let mut dist = 256.0;
        let closest = *colors
            .iter()
            .reduce(|acc, e| {
                let d = color_distance(&c.color, e);
                if dist > d {
                    dist = d;
                    e
                } else {
                    acc
                }
            })
            .unwrap();
        colors.remove(&closest);
        let mut country_file = ISO_8859_1.decode(
            std::fs::read(game_dir.join("common").join(&c.path))?.as_slice(),
            encoding::DecoderTrap::Strict,
        )?;
        country_file.replace_range(
            get_color_range_in_file(&country_file).unwrap(),
            &format!("color = {{ {} {} {} }}", closest.0, closest.1, closest.2),
        );
        println!("Writing to: {}", output_dir.join(&c.path).display());
        std::fs::create_dir_all(output_dir.join(&c.path).parent().unwrap())?;
        std::fs::write(
            output_dir.join(&c.path),
            ISO_8859_1.encode(&country_file, encoding::EncoderTrap::Strict)?,
        )?;
    }
    println!("DONE");
    Ok(())
}

fn get_color_range_in_file(input: &str) -> Option<std::ops::Range<usize>> {
    let start = input.find("color")?;
    let end = start + input[start..].find('}')? + '}'.len_utf8();
    Some(start..end)
}

fn get_country_tags_files(game_dir: &Path) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    Ok(game_dir
        .join("common/country_tags")
        .read_dir()?
        .filter_map(|r| r.ok())
        .filter_map(|e| {
            if e.file_type().unwrap().is_file() {
                if let Ok(bytes) = std::fs::read(e.path()) {
                    if let Ok(parsed) = String::from_utf8(bytes) {
                        return Some(parsed);
                    }
                }
            }
            None
        })
        .collect::<Vec<String>>())
}
