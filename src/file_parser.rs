use std::{fs::{OpenOptions}, io::Read};

use crate::{comms::Entry, colour::Colour};

pub fn parse_file(filename: String) -> Vec<([char; 5], u8)> {
    let mut string_data = String::new();

    let mut file = OpenOptions::new().read(true).open(&filename).expect(format!("**ERROR: the file ({}) could not be found**", filename).as_str());

    file.read_to_string(&mut string_data).expect(format!("**ERROR: Could not read {}**", filename).as_str());
    
    let mut data:  Vec<([char; 5], u8)> = Vec::new();

    let lines = string_data.lines();

    let mut entry: ([char; 5], u8) = (['W'; 5], 0);

    for (i, line) in lines.into_iter().enumerate() {
        
        if i % 2 == 0 {
            // is a colour set
            let mut colour_set = ['W'; 5];

            for (i, char) in line.to_string().chars().enumerate() {
                colour_set[i] = char;
            }

            entry.0 = colour_set;
        }
        else {
            entry.1 = line.to_string().parse::<u8>().expect(format!("**ERROR: The incidence at line {} in the file {} is not an unsigned 8-bit integer**", i, filename).as_str());
            data.push(entry);
        }   
    }

    data
}

pub fn simplify_output(entries: Vec<Entry>) -> String {
    let mut file_output = String::new();

    for entry in entries {
        match entry.SUB {
            // SNC
            1 => {
                match entry.IST {
                    3 => {
                        match entry.Dec {
                            0 => file_output = format!("{}SNC: MARV must go forward\n", file_output),
                            1 => file_output = format!("{}SNC: MARV must reverse\n", file_output),
                            2 => file_output = format!("{}SNC: MARV must rotate {} deg left\n", file_output, (((entry.Dat1 as u16) << 8) + entry.Dat0 as u16)),
                            3 => file_output = format!("{}SNC: MARV must rotate {} deg right\n", file_output, (((entry.Dat1 as u16) << 8) + entry.Dat0 as u16)),
                            _ => panic!("Something has gone terribly wrong")
                        }
                    }
                    _ => {},
                }
            },
            // MDPS
            2 => {
                match entry.IST {
                    2 => {
                        match entry.Dec {
                            0 => file_output = format!("{}MDPS: last measured rotation is {} deg\n", file_output, (((entry.Dat1 as u16) << 8) + entry.Dat0 as u16)),
                            2 => file_output = format!("{}MDPS: last measured rotation is {} deg left\n", file_output, (((entry.Dat1 as u16) << 8) + entry.Dat0 as u16)),
                            3 => file_output = format!("{}MDPS: last measured rotation is {} deg right\n", file_output, (((entry.Dat1 as u16) << 8) + entry.Dat0 as u16)),
                            _ => panic!("Something has gone terribly wrong"),
                        }
                    }
                    3 => {
                        file_output = format!("{}MDPS: MARV's right wheel speed is {} mm/s\n", file_output, entry.Dat1);
                        file_output = format!("{}MDPS: MARV's left wheel speed is {} mm/s\n", file_output, entry.Dat1);
                    }
                    4 => file_output = format!("{}MDPS: MARV's distance travelled since last stop is {} mm\n", file_output, (((entry.Dat1 as u16) << 8) + entry.Dat0 as u16)),
                    _ => {},
                }
            },
            // SS
            3 => {
                match entry.IST {
                    1 => {
                        let colours = get_colours(((entry.Dat1 as u16) << 8) + entry.Dat0 as u16);
                        file_output = format!("{}SS: sees colours {:?}\n", file_output, colours);
                    },
                    2 => file_output = format!("{}SS: measured incidence {} deg\n", file_output, entry.Dat1),
                    _ => panic!("Something has gone terribly wrong"), 
                }
                
            },
            _ => panic!("Something has gone terribly wrong"), 
        }
    }

    file_output
}

fn get_colours(colour_word: u16) -> [Colour; 5] {
    [
        Colour::from((colour_word & 0b0111000000000000) >> 12),
        Colour::from((colour_word & 0b0000111000000000) >> 9),
        Colour::from((colour_word & 0b0000000111000000) >> 6),
        Colour::from((colour_word & 0b0000000000111000) >> 3),
        Colour::from(colour_word & 0b0000000000000111),
    ]
}