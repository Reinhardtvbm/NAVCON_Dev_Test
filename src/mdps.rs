use std::time::Instant;

use crate::comms::{Packet};

pub struct Mdps {
    current_packet: Packet,
    left_wheel_speed: u8,
    right_wheel_speed: u8,
    distance: u16,
    rotation: u16,
    time: Instant
}

impl Mdps {
    pub fn new() -> Self {
        Self { 
            current_packet: Packet::new(),
            left_wheel_speed: 0, 
            right_wheel_speed: 0, 
            distance: 0, 
            rotation: 0,
            time: Instant::now()
        }
    }

    pub fn is_stopped(&self) -> bool {
        self.right_wheel_speed == 0 && self.left_wheel_speed == 0
    }

    pub fn update_distance(&mut self) {
        self.distance += ((self.right_wheel_speed as f32)*(self.time.elapsed().as_nanos() as f32/1000_000_000.0)).round() as u16;
        self.time = Instant::now();
    }

    pub fn set_rotation(&mut self, rotation: u16) {
        self.rotation = rotation;
    }

    pub fn reset_distance(&mut self) {
        self.distance = 0;
    }

    pub fn set_left_wheel_speed(&mut self, left_wheel_speed: u8) {
        self.left_wheel_speed = left_wheel_speed;
    }

    pub fn set_right_wheel_speed(&mut self, right_wheel_speed: u8) {
        self.right_wheel_speed = right_wheel_speed;
    }
}