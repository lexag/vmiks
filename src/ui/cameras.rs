use crate::{
    app::VMiksApp,
    project::model::Camera,
    ui::path_picker::{DefaultIconProvider, PathPicker},
};
use egui::Widget;
use std::path::PathBuf;

pub fn cameras_window(app: &mut VMiksApp, ctx: &egui::Context) {
    let mut window_open = app.windows.cameras;
    egui::Window::new("Cameras")
        .open(&mut window_open)
        .min_size([340.0, 200.0])
        .show(ctx, |ui| {
            egui::Grid::new("cameras-grid")
                .min_col_width(100.0)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("id");
                    ui.label("name");
                    ui.label("path");
                    ui.label("offset");
                    ui.end_row();

                    for camera in &mut app.project.cameras {
                        ui.text_edit_singleline(&mut camera.id);
                        ui.text_edit_singleline(&mut camera.name);

                        let mut path_s = camera.path.to_str().unwrap_or_default().to_owned();
                        PathPicker::<_, DefaultIconProvider>::new(&mut path_s, &PathBuf::from("."))
                            .ui(ui);
                        camera.path = PathBuf::from(path_s);

                        egui::DragValue::new(&mut camera.offset).suffix('s').ui(ui);

                        ui.end_row();
                    }
                });

            if ui.button("New Camera").clicked() {
                app.project.cameras.push(Camera::default());
            }
        });

    app.windows.cameras = window_open;
}
