use std::time::Instant;

use serialport::SerialPort;

use crate::{
    auxiliary::AdjacentBytes,
    comms::{send_packet, ControlByte, Packet},
};

enum State {
    Idle,
    Forward,
    Reverse,
    RotateRight,
    RotateLeft,
    Stop,
}

pub struct Mdps {
    state: State,
    left_wheel_speed: u8,
    right_wheel_speed: u8,
    pub distance: u16,
    rotation: u16,
    time: Instant,
}

impl Mdps {
    pub fn new() -> Self {
        Self {
            state: State::Idle,
            left_wheel_speed: 0,
            right_wheel_speed: 0,
            distance: 0,
            rotation: 0,
            time: Instant::now(),
        }
    }

    fn reset_time(&mut self) {
        self.time = Instant::now();
    }

    fn update_distance(&mut self) {
        self.distance += ((self.right_wheel_speed as f32)
            * (self.time.elapsed().as_nanos() as f32 / 1000_000_000.0))
            .round() as u16;
        self.reset_time();
    }

    fn set_rotation(&mut self, rotation: u16) {
        self.rotation = rotation;
    }

    pub fn reset_distance(&mut self) {
        self.distance = 0;
    }

    fn reset_rotation(&mut self) {
        self.rotation = 0;
    }

    pub fn set_left_wheel_speed(&mut self, left_wheel_speed: u8) {
        self.left_wheel_speed = left_wheel_speed;
    }

    pub fn set_right_wheel_speed(&mut self, right_wheel_speed: u8) {
        self.right_wheel_speed = right_wheel_speed;
    }

    fn update_nav_data(&mut self, navcon_data: Packet) {
        match navcon_data.dec() {
            // Forward
            0 => {
                self.set_left_wheel_speed(navcon_data.dat1());
                self.set_right_wheel_speed(navcon_data.dat0());

                if navcon_data.dat0() == 0 && navcon_data.dat1() == 0 {
                    self.state = State::Stop;
                } else {
                    self.state = State::Forward;
                }
            }
            // Reverse
            1 => {
                self.set_left_wheel_speed(navcon_data.dat1());
                self.set_right_wheel_speed(navcon_data.dat0());
                self.state = State::Reverse;
            }
            // Rotate Left
            2 => {
                self.set_rotation(u16::from(AdjacentBytes::make(
                    navcon_data.dat1(),
                    navcon_data.dat0(),
                )));
                self.state = State::RotateLeft;
            }
            // Rotate Right
            3 => {
                self.set_rotation(u16::from(AdjacentBytes::make(
                    navcon_data.dat1(),
                    navcon_data.dat0(),
                )));
                self.state = State::RotateRight;
            }
            _ => {}
        }
    }

    pub fn update_maze_state(&mut self, navcon_data: Packet, port: &mut Box<dyn SerialPort>) {
        self.update_nav_data(navcon_data);
        let mut dec = 2;
        match self.state {
            State::Idle => self.reset_time(),
            State::Stop => {
                self.reset_distance();
                self.reset_rotation();
            }
            State::Forward => {
                self.update_distance();
            }
            State::Reverse => {
                self.update_distance();
            }
            State::RotateLeft => dec = 2,
            State::RotateRight => dec = 3,
        }

        let mut out_packet = Packet::from([0, 0, 0, 0]);

        send_packet(ControlByte::MLevel, &mut out_packet, port);

        let rotation_bytes = AdjacentBytes::from(self.rotation);

        out_packet = Packet::from([0, rotation_bytes.get_msb(), rotation_bytes.get_lsb(), dec]);
        send_packet(ControlByte::MRotation, &mut out_packet, port);

        out_packet = Packet::from([0, self.left_wheel_speed, self.right_wheel_speed, 0]);
        send_packet(ControlByte::MSpeed, &mut out_packet, port);

        let distance_bytes = AdjacentBytes::from(self.distance);

        out_packet = Packet::from([0, distance_bytes.get_msb(), distance_bytes.get_lsb(), 0]);
        send_packet(ControlByte::MDistance, &mut out_packet, port);
    }
}
