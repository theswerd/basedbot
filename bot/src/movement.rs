use std::{collections::BTreeMap, sync::Arc};

use crate::humanoid::Joint;

#[derive(Clone)]
pub struct Frame {
    pub joints: BTreeMap<Joint, f32>,
}

impl PartialEq for Frame {
    fn eq(&self, other: &Self) -> bool {
        self.joints == other.joints
    }
}
impl Eq for Frame {}

pub struct State {
    current: Option<Frame>,
    queue: Arc<crossbeam::queue::SegQueue<Frame>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            current: None,
            queue: Arc::new(crossbeam::queue::SegQueue::new()),
        }
    }

    pub fn push_frame(&self, frame: Frame) {
        self.queue.push(frame);
    }
}
