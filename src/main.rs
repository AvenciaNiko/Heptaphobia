use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = &args[2];
    let option = &args[1];

    if option == "-e" {encode(file.to_string());}
    else {if option == "-d" {decode(file.to_string());}
    else {println!("Invalid Arguments");}}

}

fn encode(file: String) {
    let content = fs::read_to_string(file).expect("Awawa");
    let mut Message = String::from("0");

    for i in content.chars() {
        let mut counter = i as u32;
        
        while counter > 0 {
            Message.push('7');
            counter -= 1;
        }
        Message.push('0');
    }

    println!("{}", Message);
}

fn decode(file: String) {
    let content = fs::read_to_string(file).expect("Awawa");
    let mut Message = String::from("");
    let mut counter: u32 = 0;

    for i in content.chars() {
        if i == '0' {
            if counter != 0 {
                Message.push(char::from_u32(counter).expect("UwU"));
                counter = 0;
            }
        } else {
            counter += 1;
        }
    }

    println!("{}", Message);
}
