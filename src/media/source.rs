use crate::{error::MiksError, media::frame::VideoFrame};
use gstreamer::ClockTime;

pub trait VideoSource {
    fn seek(&mut self, position: f64) -> Result<(), MiksError>;
    fn step(&mut self) -> Result<(), MiksError>;
    fn play(&mut self) -> Result<(), MiksError>;
    fn pause(&mut self) -> Result<(), MiksError>;

    fn duration(&self) -> f64;
    fn latest_frame(&self) -> &VideoFrame;
    fn time(&self) -> ClockTime;
    fn playing(&self) -> bool;
}
