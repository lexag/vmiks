use crate::switcher::decision::DecisionList;
use std::path::PathBuf;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Project {
    version: u64,
    metadata: ProjectMetadata,
    cameras: Vec<Camera>,
    decisions: DecisionList,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ProjectMetadata {
    name: String,
    duration: f64,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Camera {
    id: String,
    name: String,
    path: PathBuf,
    offset: f64,
}
