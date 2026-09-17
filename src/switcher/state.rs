use egui::Key;

#[derive(Debug, Default)]
pub struct SwitcherState {
    pub preview: usize,
    pub program: usize,
    pub transition: Transition,
}

#[derive(Debug, Default)]
pub enum Transition {
    #[default]
    Cut,
    Dissolve {
        duration: f64,
    },
}

impl SwitcherState {
    pub fn update(&mut self, ctx: &egui::Context) {
        for (i, key) in [Key::Num1, Key::Num2, Key::Num3, Key::Num4, Key::Num5]
            .iter()
            .enumerate()
        {
            if ctx.input(|i| i.key_pressed(*key)) {
                self.preview = i
            };
        }

        if ctx.input(|i| i.key_pressed(Key::Enter)) {
            self.take();
        }
    }

    fn take(&mut self) {
        std::mem::swap(&mut self.program, &mut self.preview);
    }
}
