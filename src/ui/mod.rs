use crate::{
    app::VMiksApp,
    ui::{cameras::cameras_window, control::control_bar, multiview::multiview, program::program},
};

mod cameras;
mod control;
mod multiview;
mod path_picker;
mod program;
mod project;
mod timeline;

pub fn application_ui(ctx: &egui::Context, frame: &mut eframe::Frame, app: &mut VMiksApp) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        // The top panel is often a good place for a menu bar:

        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
            ui.menu_button("View", |ui| {
                if ui.button("Cameras").clicked() {
                    app.windows.cameras = !app.windows.cameras;
                }
            });
            ui.add_space(16.0);

            egui::widgets::global_theme_preference_buttons(ui);
            egui::warn_if_debug_build(ui);
        });
    });

    cameras_window(app, ctx);

    app.playback
        .report_delta_time(ctx.input(|i| i.stable_dt as f64));

    ctx.request_repaint();

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical(|ui| {
            //program(app, ui);
            multiview(app, ui);
            control_bar(app, ui);
        });
    });
}
