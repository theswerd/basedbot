use std::{
    collections::BTreeMap,
    sync::Arc,
    time::{Duration, Instant},
};

use crate::humanoid::Joint;

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub joints: BTreeMap<Joint, f32>,
}

struct CurrentFrame {
    frame: Frame,
    duration: Duration,
}

pub struct MovementState {
    current: Option<CurrentFrame>,
    queue: Arc<crossbeam::queue::SegQueue<Frame>>,
    last_tick: Instant,
}

impl MovementState {
    pub fn new() -> Self {
        Self {
            current: None,
            queue: Arc::new(crossbeam::queue::SegQueue::new()),
            last_tick: Instant::now(),
        }
    }

    pub fn push_frame(&self, frame: Frame) {
        self.queue.push(frame);
    }

    pub fn is_complete(&self, current_state: Frame) -> bool {
        if let Some(frame) = &self.current {
            return &frame.frame == &current_state;
        }

        return false;
    }

    pub fn step(&mut self) -> BTreeMap<Joint, f32> {
        let Some(current) = &self.current else {
            return Default::default();
        };

        // check if the current value is close enough to the target

        let elapsed = self.last_tick.elapsed();

        let ratio = elapsed.as_millis() as f32 / current.duration.as_millis() as f32;

        let frame = current
            .frame
            .joints
            .iter()
            .map(|(joint, val)| (joint.clone(), val * ratio))
            .collect();

        self.last_tick = Instant::now();

        frame
    }
}
