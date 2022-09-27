use crate::{
    colour::Colour,
    comms::{get_packet, Packet, PacketNo},
    mdps::Mdps,
    ss::SS,
};
use serialport::SerialPort;

pub fn run_navcon_with(colour: Colour, incidence: u8, port: &mut Box<dyn SerialPort>) {
    let isd = 10f64;

    let mut motor_system = Mdps::new();
    let mut sensor_system = SS::with_data([Colour::White; 5], 0);

    // send white until some distance travelled
    while motor_system.distance < 50 {
        let snc_packets = get_snc_packets(port);

        motor_system.update_maze_state(snc_packets[PacketNo::Navigate], port);
        sensor_system.send_data(port);
    }

    sensor_system = SS::with_data(
        [
            colour,
            Colour::White,
            Colour::White,
            Colour::White,
            Colour::White,
        ],
        0,
    );

    for _i in 0..5 {
        let snc_packets = get_snc_packets(port);
        motor_system.update_maze_state(snc_packets[PacketNo::Navigate], port);
        sensor_system.send_data(port);
    }

    let distance_to_next_colour = (isd * (incidence as f64).tan()).round() as u16;
    let mut sensor_system = SS::with_data([Colour::White; 5], 0);

    while motor_system.distance < 50 + distance_to_next_colour {
        let snc_packets = get_snc_packets(port);
        motor_system.update_maze_state(snc_packets[PacketNo::Navigate], port);
        sensor_system.send_data(port);
    }

    sensor_system = SS::with_data(
        [
            Colour::White,
            colour,
            Colour::White,
            Colour::White,
            Colour::White,
        ],
        incidence,
    );

    let snc_packets = get_snc_packets(port);
    motor_system.update_maze_state(snc_packets[PacketNo::Navigate], port);
    sensor_system.send_data(port);
}

fn get_snc_packets(port: &mut Box<dyn SerialPort>) -> [Packet; 3] {
    let mut packets = [Packet::new(); 3];

    get_packet(&mut packets[0], port);

    if packets[0].control_byte() != 145 {
        println!("ERROR: Expected CLAPSNAP, but got above packet...");
    }

    get_packet(&mut packets[1], port);

    if packets[1].control_byte() != 146 {
        println!("ERROR: Expected BUTTON, but got above packet...");
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
