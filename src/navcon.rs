use serialport::SerialPort;
use tabled::{Table, Style};
use crate::{comms::{get_packet, send_packet, ControlByte, Packet, Entry}, ss::SS, file_parser::simplify_output};


pub fn run_navcon_with(colours: [char; 5], incidence: u8, port: &mut Box<dyn SerialPort>) -> (String, String) {
    println!("NAVCON:");
    
    let mut table_entries: Vec<Entry> = Vec::new();

    let mut in_packet = Packet::new();
    let mut out_packet = Packet::new();

    let mut sensor = SS::new();
    sensor.set_sensor_colours(colours);
    sensor.set_incidence(incidence);

    table_entries.push(get_packet(&mut in_packet, port));

    if in_packet.control_byte() != 145{
        println!("ERROR: Expected CLAPSNAP, but got above packet...")
    }

    table_entries.push(get_packet(&mut in_packet, port));

    if in_packet.control_byte() != 146{
        println!("ERROR: Expected BUTTON, but got above packet...")
    }

    table_entries.push(get_packet(&mut in_packet, port));

    if in_packet.control_byte() != 147{
        println!("ERROR: Expected NAVCON, but got above packet...")
    }

    table_entries.push(send_packet(ControlByte::MLevel, &mut out_packet, port));
    table_entries.push(send_packet(ControlByte::MRotation, &mut out_packet, port));
    table_entries.push(send_packet(ControlByte::MSpeed, &mut out_packet, port));
    table_entries.push(send_packet(ControlByte::MDistance, &mut out_packet, port));

    let colour_word = sensor.get_colour_word();

    out_packet.set_dat1(((colour_word & 0xFF00) >> 8) as u8);
    out_packet.set_dat0((colour_word & 0x00FF) as u8);

    table_entries.push(send_packet(ControlByte::MColours, &mut out_packet, port));

    out_packet.set_dat1(incidence);
    out_packet.set_dat0(0);

    table_entries.push(send_packet(ControlByte::MIncidence, &mut out_packet, port));

    println!("{}", table_entries.len());

    let string_output = Table::new(table_entries.iter()).with(Style::modern()).to_string();
    print!("{}", string_output);

    (string_output, simplify_output(table_entries))
}