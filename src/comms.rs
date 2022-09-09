use tabled::{TableIteratorExt, Tabled, Table, Style};

#[derive(Tabled)]
struct Heading {
    SYS: u8,
    SUB: u8,
    IST: u8,
    Dat1: u8,
    Dat0: u8,
    Dec: u8,
}

#[derive(Clone, Copy)]
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
        if ((self.bytes[0] & 0b00110000) >> 4) == 1 {
            println!("Received: ");
        }
        else {println!("Sent: ");}

        let out: Heading = Heading::from(self.clone());

        let table = Table::new([out]).with(Style::modern());

        println!("{}", table);

//[((self.bytes[0] & 0b11000000) >> 6), ((self.bytes[0] & 0b00110000) >> 4), (self.bytes[0] & 0b00001111) ,self.bytes[1], self.bytes[2], self.bytes[3]];
    }
}

impl From<Packet> for Heading {
    fn from(p: Packet) -> Self {
        Heading { 
            SYS: ((p.bytes[0] & 0b11000000) >> 6),
            SUB: ((p.bytes[0] & 0b00110000) >> 4),
            IST: (p.bytes[0] & 0b00001111) , 
            Dat1: p.bytes[1], 
            Dat0: p.bytes[2], 
            Dec: p.bytes[3] 
        }
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
    Start = 0
}