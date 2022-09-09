use serialport;
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

    let mut marv_port = serialport::new("COM6", 19200).open().expect("Failed to open port");

    let mut buffer: Vec<u8> = vec![0; 32];

    while marv_port.bytes_to_read().unwrap() > 0 {
        marv_port.read(buffer.as_mut_slice()).expect("No data in the port's buffer");
    }

    let mut out_packet = Packet::new();
    let mut in_packet = Packet::new();

    marv_port.write(&out_packet.bytes).expect("Failed to write data to the MARV... :(");
    out_packet.print();

    println!("Waiting for response...");
    
    while marv_port.bytes_to_read().unwrap() < 4 { }
    marv_port.read(&mut in_packet.bytes).expect("Failed to read bytes");
    in_packet.print();

    if in_packet.control_byte() != 16 {
        println!("ERROR: Expected BUTTON, but got above packet...")
    }

    while in_packet.dat1() == 0 {
        while marv_port.bytes_to_read().unwrap() < 4 { }
        marv_port.read(&mut in_packet.bytes).expect("Failed to read bytes");
        in_packet.print();
    }

    in_packet.print();

    println!("Calibrate:");

    out_packet.set_control_byte(ControlByte::CCalibrate);
    out_packet.print();
    marv_port.write(&out_packet.bytes).expect("Failed to write data to the MARV... :(");

    out_packet.set_control_byte(ControlByte::CVelocity);
    out_packet.print();
    marv_port.write(&out_packet.bytes).expect("Failed to write data to the MARV... :(");

    out_packet.set_control_byte(ControlByte::CLevel);
    out_packet.print();
    marv_port.write(&out_packet.bytes).expect("Failed to write data to the MARV... :(");

    out_packet.set_control_byte(ControlByte::CColours);
    out_packet.print();
    marv_port.write(&out_packet.bytes).expect("Failed to write data to the MARV... :(");

    while marv_port.bytes_to_read().unwrap() < 4 { }
    marv_port.read(&mut in_packet.bytes).expect("Failed to read bytes");
    in_packet.print();

    if in_packet.control_byte() != 80 {
        println!("ERROR: Expected BUTTON, but got above packet...")
    }

    while in_packet.dat1() == 0 {
        while marv_port.bytes_to_read().unwrap() < 4 { }
        marv_port.read(&mut in_packet.bytes).expect("Failed to read bytes");
        in_packet.print();
    }

    println!("End Calibrate");

    // main NAVCON loop
    for _i in 1..5 {
        println!("NAVCON:");
        while marv_port.bytes_to_read().unwrap() < 4 { }

        marv_port.read(&mut in_packet.bytes).expect("Failed to read bytes");
        in_packet.print();

        if in_packet.control_byte() != 145{
            println!("ERROR: Expected CLAPSNAP, but got above packet...")
        }

        while marv_port.bytes_to_read().unwrap() < 4 { }

        marv_port.read(&mut in_packet.bytes).expect("Failed to read bytes");
        in_packet.print();

        if in_packet.control_byte() != 146{
            println!("ERROR: Expected BUTTON, but got above packet...")
        }
        
        while in_packet.control_byte() != 146 && in_packet.dat1() != 1{
            if marv_port.bytes_to_read().unwrap() > 3 {
                marv_port.read(&mut in_packet.bytes).expect("Failed to read bytes");
            }
        }

        while marv_port.bytes_to_read().unwrap() < 4 { }

        marv_port.read(&mut in_packet.bytes).expect("Failed to read bytes");
        in_packet.print();

        if in_packet.control_byte() != 147{
            println!("ERROR: Expected NAVCON, but got above packet...")
        }

        out_packet.set_control_byte(ControlByte::MLevel);
        out_packet.print();
        marv_port.write(&out_packet.bytes).expect("Failed to write data to the MARV... :(");

        out_packet.set_control_byte(ControlByte::MRotation);
        out_packet.print();
        marv_port.write(&out_packet.bytes).expect("Failed to write data to the MARV... :(");

        out_packet.set_control_byte(ControlByte::MSpeed);
        out_packet.print();
        marv_port.write(&out_packet.bytes).expect("Failed to write data to the MARV... :(");

        out_packet.set_control_byte(ControlByte::MDistance);
        out_packet.print();
        marv_port.write(&out_packet.bytes).expect("Failed to write data to the MARV... :(");


        out_packet.set_control_byte(ControlByte::MColours);
        out_packet.print();
        marv_port.write(&out_packet.bytes).expect("Failed to write data to the MARV... :(");

        out_packet.set_control_byte(ControlByte::MIncidence);
        out_packet.print();
        marv_port.write(&out_packet.bytes).expect("Failed to write data to the MARV... :(");
    }
    
}
