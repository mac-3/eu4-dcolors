use std::{collections::HashSet, path::PathBuf};

pub fn read_all_files_recv(entry_point: PathBuf) -> Result<Vec<String>, std::io::Error> {
    let mut acc = vec![];
    if entry_point.is_dir() {
        let mut stack = vec![entry_point];
        while let Some(curr) = stack.pop() {
            type VecPair = Vec<(PathBuf, bool)>;
            let (dirs, files): (VecPair, VecPair) = curr
                .read_dir()?
                .filter_map(|r| r.ok())
                .map(|d| (d.path(), d.file_type().unwrap().is_dir()))
                .partition(|(_, dir)| *dir);
            stack.append(&mut dirs.into_iter().map(|(p, _)| p).collect::<Vec<PathBuf>>());
            acc.append(
                &mut files
                    .into_iter()
                    .map(|(p, _)| String::from_utf8_lossy(&std::fs::read(p).unwrap()).into_owned())
                    .collect::<Vec<String>>(),
            );
        }
    }
    Ok(acc)
}

pub fn gen_colors_set(countries: usize) -> HashSet<(u8, u8, u8)> {
    let divs = f64::cbrt(countries as f64);
    let divs_size = 256.0 / divs;
    let mut acc = HashSet::new();
    let mut c1 = 0.0f64;
    while c1 <= 255.0 {
        let mut c2 = 0.0f64;
        while c2 <= 255.0 {
            let mut c3 = 0.0f64;
            while c3 <= 255.0 {
                acc.insert((
                    (c1 + (divs_size / 2.0)).round() as u8,
                    (c2 + (divs_size / 2.0)).round() as u8,
                    (c3 + (divs_size / 2.0)).round() as u8,
                ));
                c3 += divs_size;
            }
            c2 += divs_size;
        }
        c1 += divs_size;
    }
    acc
}

pub fn color_distance(src: &(u8, u8, u8), dst: &(u8, u8, u8)) -> f32 {
    let (r1, g1, b1) = (f32::from(src.0), f32::from(src.1), f32::from(src.2));
    let (r2, g2, b2) = (f32::from(dst.0), f32::from(dst.1), f32::from(dst.2));
    f32::sqrt((r2 - r1).powi(2) + (g2 - g1).powi(2) + (b2 - b1).powi(2))
}
