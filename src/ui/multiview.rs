use crate::app::VMiksApp;
use egui::{pos2, vec2, Align2, Color32, FontId, Painter, Rect, Sense, Widget};

const ASPECT: f32 = 9.0 / 16.0;

pub fn multiview(app: &mut VMiksApp, ui: &mut egui::Ui) {
    for camera in &mut app.project.cameras {
        camera.decoder.keep_time(&app.playback, camera.offset);
        camera.decoder.update_texture();
    }
    let width = ui.available_width();
    let height = width / 2.0 * ASPECT + width / 5.0 * ASPECT;

    let (resp, p) = ui.allocate_painter(vec2(width, height), Sense::hover());

    let top_left = resp.rect.min;
    monitor_display(
        app,
        width,
        &p,
        2.0,
        top_left,
        app.switcher.preview,
        Some(Color32::GREEN),
    );
    monitor_display(
        app,
        width,
        &p,
        2.0,
        top_left + vec2(width / 2.0, 0.0),
        app.switcher.program,
        Some(Color32::RED),
    );

    const NUM_CAMS: usize = 5;

    for i in 0..NUM_CAMS {
        monitor_display(
            app,
            width,
            &p,
            NUM_CAMS as f32,
            top_left + vec2(width / NUM_CAMS as f32 * i as f32, width / 2.0 * ASPECT),
            i,
            (i == app.switcher.preview).then_some(Color32::GREEN),
        );
    }
}

fn monitor_display(
    app: &mut VMiksApp,
    width: f32,
    p: &Painter,
    width_divider: f32,
    top_left: egui::Pos2,
    camera_select: usize,
    border: Option<Color32>,
) {
    let video_rect = Rect::from_min_size(
        top_left,
        vec2(width / width_divider, width / width_divider * ASPECT),
    );

    if let Some(c) = border {
        p.rect_filled(video_rect, 0.0, c);
    }

    let video_rect = video_rect.shrink(2.0);

    let Some(camera) = app.project.cameras.get(camera_select) else {
        p.rect_filled(video_rect, 0.0, Color32::BLACK);
        p.text(
            video_rect.center(),
            Align2::CENTER_CENTER,
            "no cam",
            FontId::monospace(24.0),
            Color32::WHITE,
        );
        return;
    };
    let Some(ref tex) = camera.decoder.texture else {
        p.rect_filled(video_rect, 0.0, Color32::BLACK);
        p.text(
            video_rect.center(),
            Align2::CENTER_CENTER,
            "no cam",
            FontId::monospace(24.0),
            Color32::WHITE,
        );
        return;
    };

    p.image(
        tex.id(),
        video_rect,
        Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
        Color32::WHITE,
    );
    //p.text(
    //    video_rect.center(),
    //    Align2::CENTER_CENTER,
    //    camera_select.to_string(),
    //    FontId::monospace(150.0),
    //    Color32::GRAY.gamma_multiply(0.2),
    //);
}

//fn camera_display(p: Painter, app: &mut VMiksApp, camera_select: usize) {}
//
//fn camera_row(
//    app: &mut VMiksApp,
//    width_each: f32,
//    ui: &mut egui::Ui,
//    start_idx: usize,
//    num_cams: usize,
//) {
//    ui.horizontal(|ui| {
//        for i in start_idx..start_idx + num_cams {
//            if let Some(camera) = app.project.cameras.get_mut(i) {
//                camera_slot(width_each, ui, camera, &app.playback);
//            }
//        }
//    });
//}
//
//fn camera_slot(
//    p: Painter,
//    camera: &mut crate::project::model::Camera,
//    playback: &crate::playback::PlaybackState,
//) {
//    camera.decoder.keep_time(playback, camera.offset);
//    camera.decoder.update_texture();
//    let Some(ref tex) = camera.decoder.texture else {
//        return;
//    };
//
//    ui.vertical(|ui| {
//        egui::Image::new(tex).max_width(width_each).ui(ui);
//        ui.horizontal(|ui| {
//            ui.heading(&camera.id);
//            ui.heading(&camera.name);
//
//            //ks_common_ui::components::TextDisplay::new(&camera.id, 3).ui(ui);
//            //ks_common_ui::components::TextDisplay::new(&camera.name, 16).ui(ui);
//            //ks_common_ui::components::Button::new("Preview")
//            //    .icon(ks_common_ui::material_icons::Icon::Preview)
//            //    .indicator(None)
//            //    .ui(ui);
//            //ks_common_ui::components::Button::new("Program")
//            //    .icon(ks_common_ui::material_icons::Icon::Videocam)
//            //    .indicator(None)
//            //    .ui(ui);
//        });
//    });
//}
