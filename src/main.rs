use std::fs;
use std::process::Command;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn load() -> String {
    // 1. Specify the path to your ROM
    let file_path = "floppybird.rom"; 
    let rom_data: Vec<u8> = fs::read(file_path).expect("Failed to read the ROM file");
    let hex_string: String = rom_data
        .iter()
        .map(|byte| format!("{:02X}", byte)) // Format each byte as a 2-digit uppercase hex (e.g., "A2")
        .collect::<Vec<String>>()
        .join(" "); // Add a space between each byte for readability
    let straight_string: String = rom_data.iter().map(|byte| format!("{:02X}",byte)).collect();
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

fn term_display(grid : &Vec<Vec<u8>> ) {
    for row in grid {
        for pixel in row{
            if *pixel == 1{
                print!("#");
            }
            else {
                print!(" ");
            }
        }
        print!("\n");
    }

}



fn main() {
    let program: String = load();
    let mut mi: u16 = 0;
    let mut registers:HashMap<String, u8> = registerss();
    let mut frame: Vec<Vec<u8>> = vec![vec![0u8; 64]; 32]; // frame[y][x]
    let mut pc: u16 = 0x200;
    let mut _ra: u16;
    let mut current: &str;
    let debug: bool = false; 
//    let mem: String = fs::read("IBM.ch8").expect("Failed to read file").iter().map(|byte| format!("{:08b}", byte)).collect();
//    println!("{}",mem);


    while pc < 0x200 + (program.len() as u16) {
        current = &program[((pc as usize) - 0x200)..((pc as usize) - 0x1FC)];        
        if debug { println!("{}",&current)};
        if current == "00E0" {
            
            //clear();
        }
        else if &current[0..1] == "A"{
            mi = u16::from_str_radix(&current[1..4], 16).unwrap();
        }
        else if &current[0..1] == "6" {
            registers.insert(current[1..2].to_string(), u8::from_str_radix(&current[2..4],16).unwrap()) ;
            if debug { println!("{:?}",registers) };
        }
        else if &current[0..1] == "D" {
            registers.insert("F".to_string(),0);
            let mut j: u8 = 0;
            for i in 0..u8::from_str_radix(&current[3..4],16).unwrap() { // N is how many bytes and since the sprite is 8 wide this is also denotes how tall a sprite will be
                for bit in format!("{:08b}", u8::from_str_radix(&program[((mi as usize - 0x200) + (i as usize)) * 2 .. (((mi as usize - 0x200) + (i as usize)) * 2) + 2], 16).unwrap()).chars() { // uses mi to index program and then convert those hex character into binary for their sprite data
                    if frame[ (registers[&current[2..3]]+i) as usize ][ (registers[&current[1..2]]+j) as usize ] == 1 && bit == '1' {      // 1^1 0
                        frame[ (registers[&current[2..3]]+i) as usize ][ (registers[&current[1..2]]+j) as usize ] = 0;
                        registers.insert("F".to_string(),1);
                    }
                    else if frame[ (registers[&current[2..3]]+i) as usize ][ (registers[&current[1..2]]+j) as usize ] == 0 && bit == '1' { // 0^1 1
                        frame[ (registers[&current[2..3]]+i) as usize ][ (registers[&current[1..2]]+j) as usize ] = 1;
                    }
                    else if frame[ (registers[&current[2..3]]+i) as usize ][ (registers[&current[1..2]]+j) as usize ] == 1 && bit == '0' { // 1^0 1
                        frame[ (registers[&current[2..3]]+i) as usize ][ (registers[&current[1..2]]+j) as usize ] = 1;
                    }
                    j+=1; 

                }
                j=0;

            }
            term_display(&frame);
            thread::sleep(Duration::from_millis(500));
        }


        else if &current[0..1] == "7" {
            registers.insert( (&current[1..2]).to_string() , u8::from_str_radix(&current[2..4], 16).unwrap() + registers[&current[1..2]]);
            if debug { println!("{:?}",registers) };
            
        }
        else if &current[0..1] == "1" {
            pc = 0x200 + (( u16::from_str_radix(&current[1..4],16).unwrap() - 0x200) *2); // All the necessary shifting in order to get PC to the right next opcode
            if debug {println!("PC {}",pc)};
            if debug {println!("asdasd{}", &program[((pc as usize) - 0x200)..((pc as usize) - 0x1FC)] ) };
            pc -= 0x4;
            //panic!("help")


        }
        else if &current[0..1] == "3" {
            if registers[&current[1..2].to_string()] != u8::from_str_radix(&current[2..4], 16).unwrap() {
            }
            else {
                pc+=0x4;
            }
        }
        else if &current[0..1] == "4" {
            if registers[&current[1..2].to_string()] == u8::from_str_radix(&current[2..4], 16).unwrap() {
            }
            else {
                pc+=0x4;
            }
        }
        else if &current[0..1] == "5" {
            if registers[&current[1..2].to_string()] != registers[&current[2..3].to_string()] {
            }
            else {
                pc+=0x4;
            }
        }
        else if &current[0..1] == "6" {
            registers.insert(current[1..2].to_string(), u8::from_str_radix(&current[2..4], 16).unwrap() );
        }
        else if &current[0..1] == "8" && &current[3..4] == "0"{
            registers.insert(current[1..2].to_string(), registers[&current[2..3]]);
        }
        else if &current[0..1] == "8" && &current[3..4] == "1"{
            registers.insert(current[1..2].to_string(), registers[&current[1..2]] | registers[&current[2..3]]);
        }
        else if &current[0..1] == "8" && &current[3..4] == "2"{
            registers.insert(current[1..2].to_string(), registers[&current[1..2]] & registers[&current[2..3]]);
        }
        else if &current[0..1] == "8" && &current[3..4] == "3"{
            registers.insert(current[1..2].to_string(), registers[&current[1..2]] ^ registers[&current[2..3]]);
        }
        else if &current[0..1] == "8" && &current[3..4] == "4" {
            let (result, carry) = registers[&current[1..2]].overflowing_add(registers[&current[3..4]]);
            registers.insert(current[0..1].to_string(), result);
            if carry {
                registers.insert("F".to_string(), 1);
            }
        }
        pc+=0x4;
    }
    
}
