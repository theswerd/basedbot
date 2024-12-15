use std::{ops::Deref, sync::Arc, time::Duration};

use crossbeam::atomic::AtomicCell;
use tokio::sync::Mutex;

use crate::{Humanoid, Joint};

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub joints: std::collections::BTreeMap<Joint, f32>,
}

pub struct FrameQueue {
    current: AtomicCell<Option<Frame>>,
    // unfortunately need arc here due to axum constraints needing H: Send if we clone the whole
    // runtime
    pub queue: crossbeam::queue::SegQueue<Frame>,
}

impl FrameQueue {
    pub fn push(&self, frame: Frame) {
        self.queue.push(frame);
    }

    pub fn overwrite(&self, frame: Frame) {
        // Clear the queue
        while let Some(_) = self.queue.pop() {}

        self.current.swap(Some(frame));
    }
}

struct RuntimeInner<H: Humanoid> {
    robot: Mutex<H>,
    queue: Arc<FrameQueue>,
}

#[derive(Clone)]
pub struct Runtime<H: Humanoid> {
    inner: Arc<RuntimeInner<H>>,
}

impl<H: Humanoid> Runtime<H> {
    pub fn new(robot: H) -> Self {
        Self {
            inner: Arc::new(RuntimeInner {
                robot: Mutex::new(robot),
                queue: Arc::new(FrameQueue {
                    current: AtomicCell::new(None),
                    queue: crossbeam::queue::SegQueue::new(),
                }),
            }),
        }
    }

    pub fn queue(&self) -> Arc<FrameQueue> {
        self.inner.queue.clone()
    }

    pub fn queue_len(&self) -> usize {
        self.inner.queue.queue.len()
    }

    pub fn overwrite(&self, frame: Frame) {
        self.inner.queue.overwrite(frame);
    }

    pub fn advance(&mut self) -> bool {
        if let Some(frame) = self.inner.queue.queue.pop() {
            self.inner.queue.current.swap(Some(frame));
            return true;
        }
        false
    }

    pub fn push_frame(&self, frame: Frame) {
        self.inner.queue.queue.push(frame);
    }

    pub fn is_complete(&self, current_state: Frame) -> bool {
        // Safety: The pointer should never be null
        if let Some(frame) = &unsafe {
            self.inner
                .queue
                .current
                .as_ptr()
                .as_ref()
                .expect("non-null ptr")
        } {
            return frame == &current_state;
        }

        return false;
    }

    pub async fn step(&mut self) -> eyre::Result<bool> {
        let current = match self.inner.queue.current.take() {
            Some(current) => {
                let frame = current.clone();


                // If we have a current frame, push it back to the queue
                // This is a hack because of the atomic cell usage
                self.inner.queue.current.store(Some(current));

                frame
            }
            None => {
                if let Some(next) = self.inner.queue.queue.pop() {
                    self.inner.queue.current.store(Some(next.clone()));
                    next
                } else {
                    return Ok(false);
                }
            }
        };

        println!("RUNNING CURRENT FRAME: {:?}", current);

        self.inner
            .robot
            .lock()
            .await
            .set_joints(current.joints.clone())
            .await
            .unwrap();

        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;

            // check if all joints are within a 5 degree of the target
            let mut done = true;
            for (joint, value) in &current.joints {
                let current = self
                    .inner
                    .robot
                    .lock()
                    .await
                    .get_joint(joint.clone())
                    .await?;
                let joint_position = self
                    .inner
                    .robot
                    .lock()
                    .await
                    .translate(joint.clone(), value.clone());
                let dist = (current.position - joint_position).abs();

                let dist_check = dist > 10.0;
                if current.speed > 10.0 && dist_check {
                    println!(
                        "Re-looping looping because {:?} is {} off, it is at {}, it wants to be at {} | {}",
                        current.joint,
                        dist,
                        current.position,
                        joint_position,
                        dist_check
                    );
                    done = false;
                }
            }

            if done {
                break;
            }
        }

        Ok(self.advance())
    }
}

impl<H: Humanoid> Deref for Runtime<H> {
    type Target = Mutex<H>;

    fn deref(&self) -> &Self::Target {
        &self.inner.robot
    }
}
