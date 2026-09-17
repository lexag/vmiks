use crate::{
    app::windows::Windows,
    playback::PlaybackState,
    project::Project,
    switcher::{decision::DecisionList, SwitcherState},
    ui::application_ui,
};
use ks_common_ui::style::load_fonts;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct VMiksApp {
    #[serde(skip)]
    pub project: Project,
    #[serde(skip)]
    pub playback: PlaybackState,
    #[serde(skip)]
    pub switcher: SwitcherState,
    #[serde(skip)]
    pub decisions: DecisionList,

    pub windows: Windows,
}

impl Default for VMiksApp {
    fn default() -> Self {
        Self {
            project: Default::default(),
            playback: Default::default(),
            switcher: Default::default(),
            decisions: Default::default(),
            windows: Default::default(),
        }
    }
}

impl VMiksApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let a: VMiksApp = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        };

        let mut ctx = cc.egui_ctx.clone();
        ks_common_ui::style::load_fonts(&mut ctx);

        a
    }
}

impl eframe::App for VMiksApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        application_ui(ctx, frame, self);
    }
}
