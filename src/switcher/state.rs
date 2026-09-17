use crate::{
    playback::{self, PlaybackState},
    project::Project,
    switcher::decision::Decision,
};
use egui::Key;

#[derive(Debug, Default)]
pub struct SwitcherState {
    pub preview: usize,
    pub program: usize,
    pub transition: Transition,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Copy, Clone)]
pub enum Transition {
    #[default]
    Cut,
    Dissolve {
        duration: f64,
    },
}

impl SwitcherState {
    pub fn update(&mut self, ctx: &egui::Context, project: &mut Project, playback: &PlaybackState) {
        for (i, key) in [Key::Num1, Key::Num2, Key::Num3, Key::Num4, Key::Num5]
            .iter()
            .enumerate()
        {
            if ctx.input(|i| i.key_pressed(*key)) {
                self.preview = i
            };
        }

        if ctx.input(|i| i.key_pressed(Key::Enter)) {
            self.take(project, playback);
        }
    }

    fn take(&mut self, project: &mut Project, playback: &PlaybackState) {
        project.decisions.push(Decision {
            time: playback.time(),
            camera: self.program,
            transition: self.transition,
        });
        std::mem::swap(&mut self.program, &mut self.preview);
    }
}
