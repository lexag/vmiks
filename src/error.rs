use gstreamer::glib;

pub enum MiksError {
    DecoderError,
    DecoderStateChangeError(gstreamer::StateChangeError),
    GlibError(glib::Error),
}

impl From<glib::Error> for MiksError {
    fn from(value: glib::Error) -> Self {
        Self::GlibError(value)
    }
}

impl From<gstreamer::StateChangeError> for MiksError {
    fn from(value: gstreamer::StateChangeError) -> Self {
        Self::DecoderStateChangeError(value)
    }
}
