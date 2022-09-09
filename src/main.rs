use std::time::Instant;

use serialport::{self, SerialPort};
use serialport::{available_ports};

use crate::comms::{Packet, ControlByte};

mod comms;

fn main() {
    print!("\n**************************************************************************************\n\nWelcome to the \"Bench\"!\nThis program simulates comms from the SS and MDPS, so that you can test the NAVCON :D\n");
    print!("======================================================================================\n\n");

    println!("Here are the available serial ports:");

    let ports = available_ports().expect("No serial ports available");

    for port in ports {
        println!("{}", port.port_name);
    }

    print!("\n\n");

    let mut marv_port = serialport::new("COM6", 19200).open().expect("Failed to open port");

    let mut buffer: Vec<u8> = vec![0; 32];

    while marv_port.bytes_to_read().unwrap() > 0 {
        marv_port.read(buffer.as_mut_slice()).expect("No data in the port's buffer");
    }

    let mut out_packet = Packet::new();
    let mut in_packet = Packet::new();

    let time = Instant::now();

    send_packet(ControlByte::Start, &mut out_packet, &mut marv_port);

    println!("Waiting for response...");

    get_packet(&mut in_packet, &mut marv_port);

    if in_packet.control_byte() != 16 {
        println!("ERROR: Expected BUTTON, but got above packet...")
    }

    while in_packet.dat1() == 0 {
        get_packet(&mut in_packet, &mut marv_port);
    }

    println!("Calibrate:");

    send_packet(ControlByte::CCalibrate, &mut out_packet, &mut marv_port);
    send_packet(ControlByte::CVelocity, &mut out_packet, &mut marv_port);
    send_packet(ControlByte::CLevel, &mut out_packet, &mut marv_port);
    send_packet(ControlByte::CColours, &mut out_packet, &mut marv_port);

    get_packet(&mut in_packet, &mut marv_port);

    if in_packet.control_byte() != 80 {
        println!("ERROR: Expected BUTTON, but got above packet...");
    }

    while in_packet.dat1() == 0 {
        get_packet(&mut in_packet, &mut marv_port);
    }

    println!("End Calibrate");

    // main NAVCON loop
    for _i in 1..50 {
        println!("NAVCON:");

        get_packet(&mut in_packet, &mut marv_port);

        if in_packet.control_byte() != 145{
            println!("ERROR: Expected CLAPSNAP, but got above packet...")
        }

        get_packet(&mut in_packet, &mut marv_port);

        if in_packet.control_byte() != 146{
            println!("ERROR: Expected BUTTON, but got above packet...")
        }

        get_packet(&mut in_packet, &mut marv_port);

        if in_packet.control_byte() != 147{
            println!("ERROR: Expected NAVCON, but got above packet...")
        }

        send_packet(ControlByte::MLevel, &mut out_packet, &mut marv_port);
        send_packet(ControlByte::MRotation, &mut out_packet, &mut marv_port);
        send_packet(ControlByte::MSpeed, &mut out_packet, &mut marv_port);
        send_packet(ControlByte::MDistance, &mut out_packet, &mut marv_port);

        send_packet(ControlByte::MColours, &mut out_packet, &mut marv_port);
        send_packet(ControlByte::MIncidence, &mut out_packet, &mut marv_port);
    }
    
    println!("time: {}s", time.elapsed().as_nanos() as f32/1000_000_000.0);

}

fn send_packet(control_byte: ControlByte, packet: &mut Packet, port: &mut Box<dyn SerialPort>) {
    packet.set_control_byte(control_byte);
    port.write(&packet.bytes).expect("Failed to write data to the MARV... :(");
    packet.print();
}

fn get_packet(packet: &mut Packet, port: &mut Box<dyn SerialPort>) {
    while port.bytes_to_read().unwrap() < 4 { }
    port.read(&mut packet.bytes).expect("Failed to read bytes");
    packet.print();
}