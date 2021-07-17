use std::{fs::File, io::BufReader};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct IncludeExcludeConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectConfig {
    #[serde(rename = "project")]
    pub project: IncludeExcludeConfig,
}

#[derive(Serialize, Deserialize)]
pub struct BuildaeConfig {
    #[serde(rename = "projects", skip_serializing_if = "Vec::is_empty")]
    pub projects: Vec<ProjectConfig>,
    #[serde(rename = "general", skip_serializing_if = "Option::is_none")]
    pub general: Option<IncludeExcludeConfig>,
}

impl BuildaeConfig {
    fn includes(&self) -> Vec<String> {
        let mut includes = Vec::new();

        &self.projects.iter().for_each(|project| {
            includes.extend(project.project.include.as_ref().unwrap().clone());
        });
        includes.extend(
            self.general
                .as_ref()
                .unwrap()
                .include
                .as_ref()
                .unwrap()
                .clone(),
        );

        includes
    }

    fn excludes(&self) -> Vec<String> {
        let mut excludes = Vec::new();

        &self.projects.iter().for_each(|project| {
            excludes.extend(project.project.exclude.as_ref().unwrap().clone());
        });
        excludes.extend(
            self.general
                .as_ref()
                .unwrap()
                .exclude
                .as_ref()
                .unwrap()
                .clone(),
        );

        excludes
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
