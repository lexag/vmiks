use crate::{error::MiksError, media::source::VideoSource};
use gstreamer::{prelude::ElementExt, Stream};

pub struct Decoder {
    stream: Stream,
}

impl Decoder {
    pub fn new() -> Result<Self, MiksError> {
        Self::init()?;
        let pipeline = gstreamer::parse::launch(
            "playbin uri=https://gstreamer.freedesktop.org/data/media/sintel_trailer-480p.webm",
        )?;
        pipeline.set_state(gstreamer::State::Playing)?;
        Ok(Self {})
    }

    pub fn init() -> Result<(), MiksError> {
        Ok(gstreamer::init()?)
    }
}

impl VideoSource for Decoder {
    fn duration(&self) -> f64 {
        todo!()
    }

    fn seek(&mut self, position: f64) -> Result<(), crate::error::MiksError> {
        todo!()
    }

    fn play(&mut self) -> Result<(), crate::error::MiksError> {
        todo!()
    }

    fn pause(&mut self) -> Result<(), crate::error::MiksError> {
        todo!()
    }

    fn latest_frame(&self) -> Option<&super::frame::VideoFrame> {
        todo!()
    }
}
