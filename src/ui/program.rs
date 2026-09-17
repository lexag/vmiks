use crate::app::VMiksApp;
use egui::{vec2, Widget};

pub fn program(app: &mut VMiksApp, ui: &mut egui::Ui) {
    let width_each = ui.available_width() / 2.0;
    ui.horizontal(|ui| {
        camera_display(app, width_each, ui, app.switcher.preview);
        camera_display(app, width_each, ui, app.switcher.program);
    });
}

fn camera_display(app: &mut VMiksApp, width_each: f32, ui: &mut egui::Ui, camera_select: usize) {
    let Some(camera) = app.project.cameras.get(camera_select) else {
        return;
    };
    let Some(ref tex) = camera.decoder.texture else {
        return;
    };
    egui::Image::new(tex)
        .fit_to_exact_size(vec2(width_each, 100000.0))
        .ui(ui);
}
