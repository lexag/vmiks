use gstreamer::glib;

#[derive(Debug)]
pub enum MiksError {
    Decoder,
    DecoderStateChange(gstreamer::StateChangeError),
    Glib(glib::Error),
    GlibBool(glib::BoolError),
    OptionNone,
}

impl From<glib::Error> for MiksError {
    fn from(value: glib::Error) -> Self {
        Self::Glib(value)
    }
}

impl From<gstreamer::StateChangeError> for MiksError {
    fn from(value: gstreamer::StateChangeError) -> Self {
        Self::DecoderStateChange(value)
    }
}

impl From<glib::BoolError> for MiksError {
    fn from(value: glib::BoolError) -> Self {
        Self::GlibBool(value)
    }
}
