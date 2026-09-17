use crate::{media::decoder::DecoderSlot, switcher::decision::DecisionList};
use egui::TextureHandle;
use std::path::PathBuf;

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct Project {
    pub version: u64,
    pub metadata: ProjectMetadata,
    pub cameras: Vec<Camera>,
    pub decisions: DecisionList,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
pub struct ProjectMetadata {
    pub name: String,
    pub duration: f64,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Camera {
    pub id: String,
    pub name: String,
    pub path: PathBuf,
    pub offset: f64,
    pub mixer_slot: u8,

    #[serde(skip)]
    pub decoder: DecoderSlot,
}

impl Camera {
    pub fn new(id: String, texture: TextureHandle) -> Self {
        Self {
            id,
            name: Default::default(),
            path: Default::default(),
            offset: Default::default(),
            mixer_slot: Default::default(),
            decoder: DecoderSlot::new(texture),
        }
    }
}
