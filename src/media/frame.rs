use gstreamer::ClockTime;

#[derive(Debug, Clone)]
pub struct VideoFrame {
    pub image: egui::ColorImage,
    pub time: ClockTime,
}
