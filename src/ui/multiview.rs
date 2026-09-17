use crate::app::VMiksApp;
use egui::Widget;

pub fn multiview(app: &mut VMiksApp, ui: &mut egui::Ui) {
    ui.horizontal_wrapped(|ui| {
        for camera in &mut app.project.cameras {
            let drift_error = camera.decoder.keep_time(&app.playback, camera.offset);
            camera.decoder.update_texture();
            let Some(ref tex) = camera.decoder.texture else {
                continue;
            };

            ui.vertical(|ui| {
                egui::Image::new(tex).ui(ui);
                ui.horizontal(|ui| ui.label(drift_error.to_string()));
            });
        }
    });
}
