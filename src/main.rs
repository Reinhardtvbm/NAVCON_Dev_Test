mod auxiliary;
mod cmd;
mod colour;
mod comms;
mod file_parser;
mod mdps;
mod navcon;
mod ss;

use serialport::available_ports;
use serialport::{self, SerialPort};
use std::fs;
use std::io::{stdin, Read};
use std::time::Instant;

use crate::cmd::get_user_input;
use crate::colour::Colour;
use crate::comms::{get_packet, send_packet, ControlByte, Packet};
use crate::file_parser::parse_file;
use crate::navcon::run_navcon_with;

fn main() {
    println!(
        "\n**************************************************************************************"
    );
    println!("Welcome to \"The Bench\"!");
    println!(
        "This program simulates comms from the SS and MDPS, so that you\ncan test your NAVCON :D\n"
    );
    println!("Author: Reinhardt von Brandis-Martini");
    println!(
        "======================================================================================"
    );
    println!("MODES:");
    println!("1. Default mode: asks for a NAVCON QTP to run, then runs it");
    println!("2. Custom Input mode: lets you type in colour sensor values \n   and an incidence value to test a case of your choosing");
    println!(
        "3. Custom Script mode: type in the name of a custom QTP textfile. \n   see README.txt"
    );
    println!(
        "======================================================================================"
    );
    println!("Here are the available serial ports:");

    let ports = available_ports().expect("No serial ports available");

    for port in ports {
        println!("{}", port.port_name);
    }

    print!("\n");
    let mut input = get_user_input("Which port are you using? [1, 2, ...]");

    let mut marv_port = serialport::new(format!("COM{}", input), 19200)
        .open()
        .expect("Failed to open port");

    input = get_user_input("Start in default mode(1)? [y/n]").to_ascii_lowercase();

    let test_data: Vec<(Colour, u8)>;

    match input {
        'y' => {
            test_data = parse_file(mode_1());
        }
        'n' => {
            let mode = get_user_input("Select a mode to run in [1-3]:");

            match mode {
                '1' => test_data = parse_file(mode_1()),
                '2' => test_data = parse_file(mode_2()),
                '3' => todo!(),
                _ => panic!("There are only three modes"),
            }
        }
        _ => panic!("Expected 'y' or 'n'"),
    }

    print!("\n\n");

    let mut buffer: Vec<u8> = vec![0; 32];

    while marv_port.bytes_to_read().unwrap() > 0 {
        marv_port
            .read(buffer.as_mut_slice())
            .expect("No data in the port's buffer");
    }

    let time = Instant::now();

    /*
     *============================================================================================================
     *       MAIN FUNCTIONALITY
     *============================================================================================================
     */

    run_touches(&mut marv_port);

    let mut results = String::new();
    let mut simple_results = String::new();

    // main NAVCON loop

    for (i, (colour_set, incidence)) in test_data.iter().enumerate() {
        let result = run_navcon_with(*colour_set, *incidence, &mut marv_port);
        results = format!("{}\n\nInput set no: {}\nCP", results, i + 1);
        simple_results = format!("{}\n\nInput set no: {}\n", simple_results, i + 1);
    }

    results = format!(
        "{}\n\n{}s",
        results,
        (time.elapsed().as_secs() as f64 + time.elapsed().subsec_millis() as f64 * 1e-3)
    );

    fs::write("results.txt", results).expect("file no work");
    fs::write("simple_results.txt", simple_results).expect("file no work");
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

fn mode_1() -> String {
    let num = get_user_input("Select a QTP to run [1-5]: ")
        .to_string()
        .parse::<i32>()
        .expect("Expexted a number");

    if num > 5 {
        panic!("Only QTP1-5 exists...");
    }

    format!("src/QTPs/Dr_Badenhorst_QTPs/QTP{}.txt", num)
}

fn mode_2() -> String {
    println!("type in the name of your custom QTP: ['filename.txt']");

    let mut input;

    input = String::new();
    stdin()
        .read_line(&mut input)
        .ok()
        .expect("No Message to read");

    format!(r"src\QTPs\Custom_QTPs\{}", input)
}
