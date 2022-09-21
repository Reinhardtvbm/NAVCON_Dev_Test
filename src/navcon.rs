use serialport::SerialPort;
use tabled::{Table, Style};
use crate::{comms::{get_packet, send_packet, ControlByte, Packet, Entry, PacketNo}, ss::SS, file_parser::simplify_output, auxiliary::AdjacentBytes, colour::Colour, mdps::Mdps};



pub fn run_navcon_with(colours: Colour, incidence: u8, port: &mut Box<dyn SerialPort>) {
    let mut snc_packets = [Packet { bytes: [1; 4] }; 3];
    let mut out_packet = Packet::new();
    let mut motor_system = Mdps::new(); 

    while snc_packets[PacketNo::Navigate].dat1() != 0 {
        let snc_packets = get_snc_packets(port);

        match snc_packets[PacketNo::Navigate].dec() {
            // Forward
            0 => {
                motor_system.set_left_wheel_speed(snc_packets[PacketNo::Navigate].dat1());
                motor_system.set_right_wheel_speed(snc_packets[PacketNo::Navigate].dat0());
            },
            // Reverse
            1 => {
                motor_system.set_left_wheel_speed(snc_packets[PacketNo::Navigate].dat1());
                motor_system.set_right_wheel_speed(snc_packets[PacketNo::Navigate].dat0());
            },
            // Rotate Left
            2 => motor_system.set_rotation(u16::from(AdjacentBytes::make(snc_packets[PacketNo::Navigate].dat1(), snc_packets[PacketNo::Navigate].dat0()))),
            // Rotate Right
            3 => motor_system.set_rotation(u16::from(AdjacentBytes::make(snc_packets[PacketNo::Navigate].dat1(), snc_packets[PacketNo::Navigate].dat0()))),
            _ => {},
        }

        if motor_system.is_stopped() {
            motor_system.reset_distance();
        }
    }
}

fn get_snc_packets(port: &mut Box<dyn SerialPort>) -> [Packet; 3] {
    let mut packets = [Packet::new(); 3];
    
    get_packet(&mut packets[0], port);

    if packets[0].control_byte() != 145{
        println!("ERROR: Expected CLAPSNAP, but got above packet...")
    }

    get_packet(&mut packets[1], port);

    if packets[1].control_byte() != 146{
        println!("ERROR: Expected BUTTON, but got above packet...")
    }

    get_packet(&mut packets[2], port);

    if packets[2].control_byte() != 147 {
        println!("ERROR: Expected NAVCON, but got above packet...");
    }

    packets
}

    //println!("NAVCON:");
    
    // let mut table_entries: Vec<Entry> = Vec::new();

    // let mut in_packet = Packet::new();
    // let mut out_packet = Packet::new();

    // let mut sensor = SS::new();
    // sensor.set_sensor_colours(colours);
    // sensor.set_incidence(incidence);

    // table_entries.push(get_packet(&mut in_packet, port));

    // if in_packet.control_byte() != 145{
    //     println!("ERROR: Expected CLAPSNAP, but got above packet...")
    // }

    // table_entries.push(get_packet(&mut in_packet, port));

    // if in_packet.control_byte() != 146{
    //     println!("ERROR: Expected BUTTON, but got above packet...")
    // }

    // table_entries.push(get_packet(&mut in_packet, port));

    // if in_packet.control_byte() != 147{
    //     println!("ERROR: Expected NAVCON, but got above packet...")
    // }

    // table_entries.push(send_packet(ControlByte::MLevel, &mut out_packet, port));
    // table_entries.push(send_packet(ControlByte::MRotation, &mut out_packet, port));
    // table_entries.push(send_packet(ControlByte::MSpeed, &mut out_packet, port));
    // table_entries.push(send_packet(ControlByte::MDistance, &mut out_packet, port));

    // let colour_word = sensor.get_colour_word();
    // let bytes = AdjacentBytes::from(colour_word);

    // out_packet.set_dat1(bytes.get_msb());
    // out_packet.set_dat0(bytes.get_lsb());

    // table_entries.push(send_packet(ControlByte::MColours, &mut out_packet, port));

    // out_packet.set_dat1(incidence);
    // out_packet.set_dat0(0);

    // table_entries.push(send_packet(ControlByte::MIncidence, &mut out_packet, port));

    // println!("{}", table_entries.len());

    // let string_output = Table::new(table_entries.iter()).with(Style::modern()).to_string();
    // print!("{}", string_output);

    // (string_output, simplify_output(table_entries))