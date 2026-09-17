use crate::{
    app::VMiksApp,
    project::model::Camera,
    ui::path_picker::{DefaultIconProvider, PathPicker},
};
use egui::{Color32, ColorImage, Widget};
use std::path::PathBuf;

pub fn cameras_window(app: &mut VMiksApp, ctx: &egui::Context) {
    let mut window_open = app.windows.cameras;
    egui::Window::new("Cameras")
        .open(&mut window_open)
        .min_size([340.0, 200.0])
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Reload sources").clicked() {
                    println!("start camera");
                    for camera in &mut app.project.cameras {
                        if let Err(e) = camera.decoder.populate(&camera.path) {
                            panic!("gstreamer creation failed: {e:?}")
                        }
                    }
                }
            });
            egui::Grid::new("cameras-grid")
                .min_col_width(100.0)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("id");
                    ui.label("name");
                    ui.label("path");
                    ui.label("offset");
                    ui.label("slot");
                    ui.end_row();

                    for camera in &mut app.project.cameras {
                        ui.text_edit_singleline(&mut camera.id);
                        ui.text_edit_singleline(&mut camera.name);

                        let mut path_s = camera.path.to_str().unwrap_or_default().to_owned();
                        PathPicker::<_, DefaultIconProvider>::new(&mut path_s, &PathBuf::from("."))
                            .ui(ui);
                        camera.path = PathBuf::from(path_s);

                        egui::DragValue::new(&mut camera.offset).suffix('s').ui(ui);

                        egui::DragValue::new(&mut camera.mixer_slot).ui(ui);

                        ui.end_row();
                    }
                });

            if ui.button("New Camera").clicked() {
                let id = (app.project.cameras.len() + 1).to_string();
                let tex = ctx.load_texture(
                    &id,
                    ColorImage::filled([340, 200], Color32::MAGENTA),
                    egui::TextureOptions::default(),
                );
                app.project.cameras.push(Camera::new(id, tex));
            }
        });

    app.windows.cameras = window_open;
}
