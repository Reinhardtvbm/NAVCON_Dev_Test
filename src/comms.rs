
pub struct Packet {
    pub bytes: [u8; 4]
}

impl Packet {
    pub fn new() -> Self {
        return Self{
            bytes: [0; 4],
        };
    }

    pub fn control_byte(&self) -> u8 {
        return self.bytes[0];
    }

    pub fn dat1(&self) -> u8 {
        return self.bytes[1];
    }

    pub fn dat0(&self) -> u8 {
        return self.bytes[2];
    }

    pub fn dec(&self) -> u8 {
        return self.bytes[3];
    }

    pub fn set_control_byte(&mut self, c_byte: ControlByte) {
        self.bytes[0] = c_byte as u8;
    }

    pub fn print(&self) {
        println!("|| ({}-{}-{}) | {} | {} | {} ||", ((self.bytes[0] & 0b11000000) >> 6), ((self.bytes[0] & 0b00110000) >> 4), (self.bytes[0] & 0b00001111) ,self.bytes[1], self.bytes[2], self.bytes[3]);
    }
}

pub enum ControlByte {
    CCalibrate = 112,
    CColours = 113,
    MColours = 177,
    MIncidence = 178,
    // MEndOfMaze = 179,
    CVelocity = 96,
    CLevel = 97,
    MLevel = 161,
    MRotation = 162,
    MSpeed = 163,
    MDistance = 164,
    // SSos = 208,
    // Start = 0
}