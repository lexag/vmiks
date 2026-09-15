use crate::playback::clock::ShowClock;

mod clock;

#[derive(Debug, Default)]
pub struct PlaybackState {
    clock: ShowClock,
}

impl PlaybackState {
    pub fn play(&mut self) {
        self.clock.playing = true;
    }

    pub fn pause(&mut self) {
        self.clock.playing = false;
    }

    pub fn seek(&mut self, destination: f64) {
        if destination > 0.0 {
            self.clock.position = destination
        } else {
            self.clock.position = 0.0;
        }
    }

    pub fn jump_forward(&mut self, jump_length: f64) {
        self.clock.position += jump_length;
        self.seek(self.time());
    }

    pub fn set_playback_rate(&mut self, rate: f64) {
        self.clock.rate = rate;
    }

    pub fn get_playback_rate(&self) -> f64 {
        self.clock.rate
    }

    pub fn report_delta_time(&mut self, delta_time: f64) {
        if self.clock.playing {
            self.clock.position += self.clock.rate * delta_time;
        }
    }

    pub fn time(&self) -> f64 {
        self.clock.position
    }

    pub fn playing(&self) -> bool {
        self.clock.playing
    }
}
