use crate::switcher::state::Transition;
use std::cmp::Ordering;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct DecisionList {
    pub decisions: Vec<Decision>,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Decision {
    pub time: f64,
    pub camera: usize,
    pub transition: Transition,
}

impl DecisionList {
    pub fn push(&mut self, decision: Decision) {
        self.decisions.push(decision);
        self.decisions.sort_by(|a, b| {
            if a.time > b.time {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
    }
}
