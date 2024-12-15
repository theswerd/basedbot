use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
    time::Duration,
};

use crate::{Humanoid, Joint};

#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub joints: std::collections::BTreeMap<Joint, f32>,
}

pub struct Runtime<H: Humanoid> {
    robot: H,
    current: Option<Frame>,
    pub queue: Arc<crossbeam::queue::SegQueue<Frame>>,
}

impl<H: Humanoid> Runtime<H> {
    pub fn new(robot: H) -> Self {
        Self {
            robot,
            current: None,
            queue: Arc::new(crossbeam::queue::SegQueue::new()),
        }
    }

    pub fn advance(&mut self) -> bool {
        if let Some(frame) = self.queue.pop() {
            self.current.replace(frame);
            return true;
        }
        false
    }

    pub fn push_frame(&self, frame: Frame) {
        self.queue.push(frame);
    }

    pub fn is_complete(&self, current_state: Frame) -> bool {
        if let Some(frame) = &self.current {
            return frame == &current_state;
        }

        return false;
    }

    pub async fn step(&mut self) -> eyre::Result<bool> {
        let current = match self.current.clone() {
            Some(current) => current,
            None => {
                if let Some(next) = self.queue.pop() {
                    self.current = Some(next.clone());
                    next
                } else {
                    return Ok(false);
                }
            }
        };

        println!("RUNNING CURRENT FRAME: {:?}", current);
        self.robot.set_joints(current.joints.clone()).await.unwrap();

        loop {
            tokio::time::sleep(Duration::from_millis(100)).await;

            // check if all joints are within a 5 degree of the target
            let mut done = true;
            for (joint, value) in &current.joints {
                let current = self.robot.get_joint(joint.clone()).await?;
                let joint_position = self.robot.translate(joint.clone(), value.clone());
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
    type Target = H;

    fn deref(&self) -> &Self::Target {
        &self.robot
    }
}

impl<H: Humanoid> DerefMut for Runtime<H> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.robot
    }
}
