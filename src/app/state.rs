use crate::{
    playback::PlaybackState,
    project::Project,
    switcher::{decision::DecisionList, SwitcherState},
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct VMiksApp {
    #[serde(skip)]
    project: Option<Project>,
    #[serde(skip)]
    playback: PlaybackState,
    #[serde(skip)]
    switcher: SwitcherState,
    #[serde(skip)]
    decisions: DecisionList,
}

impl VMiksApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}
