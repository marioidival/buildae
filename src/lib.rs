use glob::Pattern;
use std::{fs::File, io::BufReader};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct IncludeExcludeConfig {
    pub include: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct BuildaeConfig {
    pub general: IncludeExcludeConfig,
}

impl BuildaeConfig {
    fn includes(&self) -> Vec<String> {
        let mut includes = Vec::new();

        if self.general.include.is_none() {
            includes.push(String::from("*"));
            return includes;
        }

        includes.extend(self.general.include.as_ref().unwrap().clone());

        includes
    }

    fn excludes(&self) -> Vec<String> {
        let mut excludes = Vec::new();

        if self.general.exclude.is_none() {
            excludes.push(String::from("*"));
            return excludes;
        }

        excludes.extend(self.general.exclude.as_ref().unwrap().clone());

        excludes
    }
}

fn load_json(json_path: &str) -> BuildaeConfig {
    let file = File::open(json_path).expect("json file not found");
    let reader = BufReader::new(file);
    let result: BuildaeConfig = serde_json::from_reader(reader).expect("fail to serializing");
    result
}

pub fn load_patterns(json_path: &str) -> (Vec<String>, Vec<String>) {
    let buildae_config = load_json(json_path);
    (buildae_config.includes(), buildae_config.excludes())
}

pub fn has_diff(files: Vec<&str>, includes: &[String], excludes: &[String]) -> bool {
    files
        .iter()
        .filter(|file| {
            !excludes
                .iter()
                .any(|pat| Pattern::new(pat).unwrap().matches(file))
        })
        .any(|file| {
            includes
                .iter()
                .any(|pat| Pattern::new(pat).unwrap().matches(file))
        })
}
