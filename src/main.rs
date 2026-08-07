use std::fs;
use std::process::Command;
use std::collections::HashMap;

fn load() -> String {
    // 1. Specify the path to your ROM
    let file_path = "IBM.ch8"; 

    // 2. Read the file exactly as it is on the disk (Raw Bytes)
    // This returns a Vec<u8> containing the machine code.
    let rom_data: Vec<u8> = fs::read(file_path).expect("Failed to read the ROM file");
    // 3. Convert the byte array into a readable Hex String
    let hex_string: String = rom_data
        .iter()
        .map(|byte| format!("{:02X}", byte)) // Format each byte as a 2-digit uppercase hex (e.g., "A2")
        .collect::<Vec<String>>()
        .join(" "); // Add a space between each byte for readability
    let list: Vec<&str> = hex_string.split(" ").collect();
    // Print the result to the console!
//    println!("{:?}", list);
    let straight_string: String = rom_data.iter().map(|byte| format!("{:02X}",byte)).collect();
    println!("{}",straight_string);




    let list2: Vec<String> = list.iter().map(ToString::to_string).collect();

    let mut instructs: Vec<String> = Vec::new();


    let mut j = 0;
    for _i in 0..(list.len()/2){
        instructs.push(format!("{}{}", list[j],list[j+1]));
        j+=2;
    }
    println!("{:?}", instructs);
//    println!("{}",instructs.len());
    return straight_string;
}

fn clear() {
   let _ = Command::new("clear").status();
   let _ = Command::new("cls").status();
}
fn registerss() -> HashMap<String,u8> {
    let registers: HashMap<String, u8> = HashMap::from([
        ("0".to_string(), 0),("1".to_string(), 0),("2".to_string(), 0),("3".to_string(), 0),("4".to_string(), 0),("5".to_string(), 0),("6".to_string(), 0),("7".to_string(), 0),("8".to_string(), 0),("9".to_string(), 0),("A".to_string(), 0),("B".to_string(), 0),("C".to_string(), 0),("D".to_string(), 0),("E".to_string(), 0),("F".to_string(), 0) ]);
    return registers;
}


fn main() {
    let program: String = load();
    let mut mi: u16;
    let mut registers:HashMap<String, u8> = registerss();
    let mut frame: Vec<Vec<u8>> = vec![vec![0u8; 64]; 32];
    let mut pc: u16 = 0x200;
    let mut _ra: u16;
    let mut current: &str;
    //clear();
    while pc < 0x200 + (program.len() as u16) {
        current = &program[((pc as usize) - 0x200)..((pc as usize) - 0x1FC)];        
        println!("{}",current);
        if current == "00E0" {
//            clear();
        }
        if &current[0..1] == "A"{
            mi = u16::from_str_radix(&current[1..4], 16).unwrap();
            println!("{}",mi);
        }
        else if &current[0..1] == "6" {
            registers.insert(current[1..2].to_string(), u8::from_str_radix(&current[2..4],16).unwrap()) ;
            println!("{:?}",registers);


        }
        else if &current[0..1] == "D" {
            for i in 0..8 {
                for j in 0..u8::from_str_radix(&current[3..4],16).unwrap() {
                    // (vx + i, vy+j)
                    frame[ (registers[&current[1..2]]+i) as usize ][ (registers[&current[2..3]]+j) as usize ] = 1;

                }
            }
            panic!("fart")
        }


        pc+=0x4;
    }
}
