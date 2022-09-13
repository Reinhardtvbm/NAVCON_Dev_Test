use serialport::SerialPort;
use tabled::Tabled;

#[allow(non_snake_case)]

#[derive(Tabled, Debug)]
pub struct Entry {
    pub Direction: String,
    pub SYS: u8,
    pub SUB: u8,
    pub IST: u8,
    pub Dat1: u8,
    pub Dat0: u8,
    pub Dec: u8,
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

    pub fn set_dat1(&mut self, val: u8) {
        self.bytes[1] = val;
    }

    pub fn set_dat0(&mut self, val: u8) {
        self.bytes[2] = val;
    }

    // pub fn set_dec(&mut self, val: u8) {
    //     self.bytes[2] = val;
    // }

    pub fn set_control_byte(&mut self, c_byte: ControlByte) {
        self.bytes[0] = c_byte as u8;
    }
}

impl From<Packet> for Entry {
    fn from(p: Packet) -> Self {
        Entry { 
            Direction: String::from("Out"),
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

pub fn send_packet(control_byte: ControlByte, packet: &mut Packet, port: &mut Box<dyn SerialPort>) -> Entry {
    packet.set_control_byte(control_byte);
    port.write(&packet.bytes).expect("Failed to write data to the MARV... :(");
    Entry::from(*packet)
}

pub fn get_packet(packet: &mut Packet, port: &mut Box<dyn SerialPort>) -> Entry {
    while port.bytes_to_read().unwrap() < 4 { }
    port.read(&mut packet.bytes).expect("Failed to read bytes");
    let mut out = Entry::from(*packet);
    out.Direction = String::from("In");
    out
}