mod ss;
mod mdps;
mod colour;
mod comms;
mod navcon;
mod file_parser;

use std::time::Instant;
use std::io::{stdin, Read};
use serialport::{self, SerialPort};
use serialport::{available_ports};


use crate::comms::{Packet, ControlByte, get_packet, send_packet, get_char};
use crate::navcon::run_navcon_with;
use crate::file_parser::{parse_file};


fn main() {
    println!("\n**************************************************************************************");
    println!("Welcome to \"The Bench\"!");
    println!("This program simulates comms from the SS and MDPS, so that you\ncan test your NAVCON :D\n");
    println!("Author: Reinhardt von Brandis-Martini");
    println!("======================================================================================");
    println!("MODES:");
    println!("1. Default mode: asks for a NAVCON QTP to run, then runs it");
    println!("2. Custom Input mode: lets you type in colour sensor values \n   and an incidence value to test a case of your choosing");
    println!("3. Custom Script mode: type in the name of a custom QTP textfile. \n   see README.txt");
    println!("======================================================================================");
    println!("Here are the available serial ports:");

    let ports = available_ports().expect("No serial ports available");

    for port in ports {
        println!("{}", port.port_name);
    }

    print!("\n");

    println!("Which port are you using? [1/2/...]");

    let mut input = String::new();

    stdin()
    .read_line(&mut input)
    .ok()
    .expect("No Message to read");

    let mut marv_port = serialport::new(format!("COM{}", get_char(&input, 0)), 19200)
                                             .open()
                                             .expect("Failed to open port");

    println!("Start in default mode(1)? [y/n]");

    input = String::new();
    stdin().read_line(&mut input).ok().expect("No Message to read");

    let char = get_char(&input, 0).to_ascii_lowercase();

    let mut test_data: Vec<([char; 5], u8)> = Vec::new();

    match char {
        'y' => {
            print!("Select a QTP to run [1-5]: ");

            input = String::new();
            stdin().read_line(&mut input).ok().expect("No Message to read");

            let qtp = get_char(&input, 0);
            let num = String::from(qtp).parse::<i32>().unwrap();

            if num > 5 {
                panic!("Only QTP1-5 exists...");
            }
            
            test_data = parse_file(format!("QTP{}.txt", num));
        },
        'n' => todo!(),
        _ => panic!("Expected 'y' or 'n'")
    }

    print!("\n\n");

    let mut buffer: Vec<u8> = vec![0; 32];

    while marv_port.bytes_to_read().unwrap() > 0 {
        marv_port.read(buffer.as_mut_slice()).expect("No data in the port's buffer");
    }

    let time = Instant::now();

    /*
     *============================================================================================================
     *       MAIN FUNCTIONALITY
     *============================================================================================================
     */

    run_touches(&mut marv_port);
    
    // main NAVCON loop

    for (colour_set, incidence) in test_data {
        run_navcon_with(colour_set, incidence, &mut marv_port);
    }
    
    println!("time: {}s", time.elapsed().as_nanos() as f32/1000_000_000.0);

}

fn run_touches(port: &mut Box<dyn SerialPort>) {
    let mut out_packet = Packet::new();
    let mut in_packet = Packet::new();

    send_packet(ControlByte::Start, &mut out_packet, port);

    println!("Waiting for response...");

    get_packet(&mut in_packet, port);

    if in_packet.control_byte() != 16 {
        println!("ERROR: Expected BUTTON, but got above packet...")
    }

    while in_packet.dat1() == 0 {
        get_packet(&mut in_packet, port);
    }

    println!("Calibrate:");

    send_packet(ControlByte::CCalibrate, &mut out_packet, port);
    send_packet(ControlByte::CVelocity, &mut out_packet, port);
    send_packet(ControlByte::CLevel, &mut out_packet, port);
    send_packet(ControlByte::CColours, &mut out_packet, port);

    get_packet(&mut in_packet, port);

    if in_packet.control_byte() != 80 {
        println!("ERROR: Expected BUTTON, but got above packet...");
    }

    while in_packet.dat1() == 0 {
        get_packet(&mut in_packet, port);
    }

    println!("End Calibrate");
}