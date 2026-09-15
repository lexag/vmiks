#[derive(Debug)]
pub struct ShowClock {
    pub position: f64,
    pub playing: bool,
    pub rate: f64,
}

impl Default for ShowClock {
    fn default() -> Self {
        Self {
            position: 0.0,
            playing: false,
            rate: 1.0,
        }
    }
}
