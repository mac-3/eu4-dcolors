use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
    path::{Path, PathBuf},
};

use crate::utils::read_all_files_recv;

#[derive(Debug)]
pub struct CountryTags {
    pub list: HashSet<CountryTag>,
}

#[derive(Debug, Clone, Eq)]
pub struct CountryTag {
    pub tag: String,
    pub color: (u8, u8, u8),
    pub path: String,
}

impl PartialEq for CountryTag {
    fn eq(&self, other: &Self) -> bool {
        self.tag == other.tag
    }
}

impl CountryTag {
    fn parse_line(line: &str, game_dir: &Path) -> CountryTag {
        let tag = &line[..3];
        let start = line.find('"').unwrap() + '"'.len_utf8();
        let end = start + (&line[start + '"'.len_utf8()..]).find('"').unwrap();
        let path = &line[start..=end];
        let country_file = game_dir.join("common").join(path);
        CountryTag {
            tag: tag.to_string(),
            color: CountryTag::parse_color_from_file(&country_file),
            path: path.to_string(),
        }
    }

    fn parse_color_from_file(file: &Path) -> (u8, u8, u8) {
        let content = String::from_utf8_lossy(std::fs::read(file).unwrap().as_slice()).into_owned();
        let start = content.find("color").unwrap();
        let start = start + content[start..].find('{').unwrap();
        let end = start + content[start..].find('}').unwrap();
        let mut c_iter = content[start..end]
            .split(char::is_whitespace)
            .filter_map(|x| x.parse::<u8>().ok());
        (
            c_iter.next().unwrap(),
            c_iter.next().unwrap(),
            c_iter.next().unwrap(),
        )
    }
}

impl std::hash::Hash for CountryTag {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.tag.hash(state);
    }
}

impl CountryTags {
    pub fn parse_files(
        input: Vec<impl AsRef<str>>,
        game_dir: &Path,
    ) -> Result<CountryTags, Box<dyn std::error::Error>> {
        let list = input
            .iter()
            .map(|e| e.as_ref().lines())
            .flatten()
            .filter(|l| l.contains('=') && !l.starts_with('#'))
            .map(|l| CountryTag::parse_line(l, game_dir))
            .collect::<HashSet<CountryTag>>();
        Ok(CountryTags { list })
    }

    pub fn process_priority_queue(
        &self,
        dirs: Vec<PathBuf>,
    ) -> Result<Vec<CountryTag>, Box<dyn std::error::Error>> {
        let mut occurrences_map: HashMap<CountryTag, usize> = HashMap::new();
        self.list.iter().for_each(|c| {
            let _ = occurrences_map.insert(c.clone(), 0);
        });
        let contents = dirs
            .into_iter()
            .map(read_all_files_recv)
            .filter_map(|r| r.ok())
            .flatten()
            .collect::<Vec<String>>();
        self.list.iter().for_each(|c| {
            contents.iter().for_each(|f| {
                occurrences_map
                    .get_mut(c)
                    .unwrap()
                    .add_assign(f.as_str().matches(c.tag.as_str()).count());
            });
        });
        let mut res = occurrences_map
            .into_iter()
            .collect::<Vec<(CountryTag, usize)>>();
        res.sort_by(|(_, a), (_, b)| (*a).cmp(b).reverse());
        Ok(res.into_iter().map(|(c, _)| c).collect::<Vec<CountryTag>>())
    }
}
