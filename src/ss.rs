use serialport::SerialPort;

use crate::{
    auxiliary::AdjacentBytes,
    colour::Colour,
    comms::{send_packet, ControlByte, Packet},
};

pub struct SS {
    sensor: [Colour; 5],
    incidence: u8,
}

impl SS {
    pub fn with_data(_sensor: [Colour; 5], _incidence: u8) -> Self {
        Self {
            sensor: _sensor,
            incidence: _incidence,
        }
    }

    pub fn get_colour_word(&self) -> u16 {
        (((self.sensor[0] as u16) << 12)
            + ((self.sensor[1] as u16) << 9)
            + ((self.sensor[2] as u16) << 6)
            + ((self.sensor[3] as u16) << 3)
            + (self.sensor[4] as u16)) as u16
    }

    pub fn send_data(&self, port: &mut Box<dyn SerialPort>) {
        let mut out_packet = Packet::from([0, 0, 0, 0]);

        let bytes = AdjacentBytes::from(self.get_colour_word());

        out_packet.set_dat1(bytes.get_msb());
        out_packet.set_dat0(bytes.get_lsb());

        send_packet(ControlByte::MColours, &mut out_packet, port);

        out_packet.set_dat1(self.incidence);
        out_packet.set_dat0(0);

        send_packet(ControlByte::MIncidence, &mut out_packet, port);
    }
}
