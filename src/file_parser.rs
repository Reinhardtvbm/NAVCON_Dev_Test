use std::{fs::File, io::Read};

pub fn parse_file(filename: String) -> Vec<([char; 5], u8)> {
    let mut string_data = String::new();

    let mut file = File::create(&filename).expect(format!("{} not found", filename).as_str());

    file.read_to_string(&mut string_data).expect(format!("Could not read {}", filename).as_str());
    
    let mut data:  Vec<([char; 5], u8)> = Vec::new();

    let lines = string_data.lines();

    for (i, line) in lines.into_iter().enumerate() {
        let mut entry: ([char; 5], u8) = (['W'; 5], 0);
        if i % 2 == 1 {
            // is a colour set
            let mut colour_set = ['W'; 5];

            for (i, char) in line.to_string().chars().enumerate() {
                colour_set[i] = char;
            }

            entry.0 = colour_set;
        }
        else {
            entry.1 = line.to_string().parse::<u8>().expect(format!("The incidence at line {} is not an unsigned 8-bit integer", i).as_str());
        }

        data.push(entry);
    }

    data
}