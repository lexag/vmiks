use crate::{error::MiksError, media::frame::VideoFrame};

pub trait VideoSource {
    fn duration(&self) -> f64;
    fn seek(&mut self, position: f64) -> Result<(), MiksError>;
    fn play(&mut self) -> Result<(), MiksError>;
    fn pause(&mut self) -> Result<(), MiksError>;
    fn latest_frame(&self) -> Option<&VideoFrame>;
}
