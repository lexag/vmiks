use crate::app::VMiksApp;
use egui::{Color32, Key, Widget};
use ks_common_ui::components::Button;

pub fn control_bar(app: &mut VMiksApp, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        let t = app.playback.time();
        let h = (t / 3600.0).floor() as u8;
        let m = (t / 60.0 % 60.0).floor() as u8;
        let s = (t % 60.0).floor() as u8;
        let ms = (t.fract() * 1000.0).floor() as u16;

        ks_common_ui::components::TextDisplay::new(
            &format!("{:>02}:{:>02}:{:>02}.{:>03}", h, m, s, ms),
            12,
        )
        .label("Program Time")
        .color_o(app.playback.playing().then_some(Color32::GREEN))
        .ui(ui);

        let button_play = Button::new("Play")
            .icon(ks_common_ui::material_icons::Icon::PlayArrow)
            .ui(ui);

        let button_pause = Button::new("Pause")
            .icon(ks_common_ui::material_icons::Icon::Pause)
            .ui(ui);

        let button_skip_b = Button::new("Skip Backward")
            .icon(ks_common_ui::material_icons::Icon::Replay5)
            .ui(ui);

        let button_skip_f = Button::new("Skip Forward")
            .icon(ks_common_ui::material_icons::Icon::Forward5)
            .ui(ui);

        if button_play.clicked()
            || ui.input(|i| i.key_pressed(Key::Space) && !app.playback.playing())
        {
            app.playback.play();
        } else if button_pause.clicked()
            || ui.input(|i| i.key_pressed(Key::Space) && app.playback.playing())
        {
            app.playback.pause();
        }

        if button_skip_f.clicked() || ui.input(|i| i.key_pressed(Key::ArrowRight)) {
            app.playback.jump_forward(5.0);
        }
        if button_skip_b.clicked() || ui.input(|i| i.key_pressed(Key::ArrowLeft)) {
            app.playback.jump_forward(-5.0);
        }
    });
}
