use serialport::SerialPort;
use crate::{comms::{get_packet, send_packet, ControlByte, Packet}, ss::SS};

pub fn run_navcon_with(colours: [char; 5], incidence: u8, port: &mut Box<dyn SerialPort>) {
    println!("NAVCON:");

    let mut in_packet = Packet::new();
    let mut out_packet = Packet::new();

    let mut sensor = SS::new();
    sensor.set_sensor_colours(colours);
    sensor.set_incidence(incidence);

    get_packet(&mut in_packet, port);

    if in_packet.control_byte() != 145{
        println!("ERROR: Expected CLAPSNAP, but got above packet...")
    }

    get_packet(&mut in_packet, port);

    if in_packet.control_byte() != 146{
        println!("ERROR: Expected BUTTON, but got above packet...")
    }

    get_packet(&mut in_packet, port);

    if in_packet.control_byte() != 147{
        println!("ERROR: Expected NAVCON, but got above packet...")
    }

    send_packet(ControlByte::MLevel, &mut out_packet, port);
    send_packet(ControlByte::MRotation, &mut out_packet, port);
    send_packet(ControlByte::MSpeed, &mut out_packet, port);
    send_packet(ControlByte::MDistance, &mut out_packet, port);

    let colour_word = sensor.get_colour_word();

    out_packet.set_dat1(((colour_word & 0xFF00) >> 8) as u8);
    out_packet.set_dat0((colour_word & 0x00FF) as u8);

    send_packet(ControlByte::MColours, &mut out_packet, port);

    out_packet.set_dat1(incidence);
    out_packet.set_dat0(0);

    send_packet(ControlByte::MIncidence, &mut out_packet, port);
}