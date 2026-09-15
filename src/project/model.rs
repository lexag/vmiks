use crate::switcher::decision::DecisionList;
use std::path::PathBuf;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Project {
    pub version: u64,
    pub metadata: ProjectMetadata,
    pub cameras: Vec<Camera>,
    pub decisions: DecisionList,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub duration: f64,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Camera {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub offset: f64,
}
