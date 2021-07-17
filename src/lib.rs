use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IncludeExcludeConfig {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub include: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exclude: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectConfig {
    #[serde(rename = "project")]
    pub project: IncludeExcludeConfig,
}

#[derive(Serialize, Deserialize)]
pub struct BuildaeConfig {
    #[serde(rename = "projects")]
    pub projects: Vec<ProjectConfig>,
    #[serde(rename = "general")]
    pub general: IncludeExcludeConfig,
}

impl BuildaeConfig {
    fn includes(&self) -> Vec<String> {
        let mut includes = Vec::new();

        &self.projects.iter().for_each(|project| {
            includes.extend(project.project.include.clone());
        });
        includes.extend(self.general.include.clone());

        includes.clone()
    }

    fn excludes(&self) -> Vec<String> {
        let mut excludes = Vec::new();

        &self.projects.iter().for_each(|project| {
            excludes.extend(project.project.exclude.clone());
        });
        excludes.extend(self.general.exclude.clone());

        excludes.clone()
    }
}

fn load_json(json_path: &str) -> BuildaeConfig {
    let file = File::open(json_path).expect("json file not found");
    let reader = BufReader::new(file);
    let result: BuildaeConfig = serde_json::from_reader(reader).expect("fail to serializing");
    result
}

pub fn load_patterns<'a>(_key: &str, json_path: &str) -> (Vec<String>, Vec<String>) {
    let buildae_config = load_json(json_path);
    (buildae_config.includes(), buildae_config.excludes())
}
