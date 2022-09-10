use crate::colour::{Colour};

pub struct SS {
    sensor : [Colour; 5],
    incidence: u8
}

impl SS {
    pub fn new() -> Self {
        Self {
            sensor: [Colour::White; 5],
            incidence: 0,
        }
    }

    pub fn get_colour_word(&self) -> u16 {
        (((self.sensor[0] as u16) << 12) 
        + ((self.sensor[1] as u16) << 9) 
        + ((self.sensor[2] as u16) << 6) 
        + ((self.sensor[3] as u16) << 3) 
        + ((self.sensor[4] as u16))) as u16
    }

    pub fn get_incidence(&self) -> u8 {
        self.incidence
    }

    pub fn set_incidence(&mut self, val: u8) {
        self.incidence = val;
    } 

    pub fn set_sensor_colours(&mut self, colours: [char; 5]) {
        for (i, colour) in colours.iter().enumerate() {
            self.sensor[i] = Colour::from_char(*colour);
        }
    }
}