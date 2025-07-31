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
        let mut counter = i as u8;
        
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
    let mut counter: u8 = 0;

    for i in content.chars() {
        if i == '0' {
            if counter != 0 {
                Message.push(counter as char);
                counter = 0;
            }
        } else {
            counter += 1;
        }
    }

    println!("{}", Message);
}
