use system_shutdown::{sleep, shutdown};
use std::{thread, time};
use std::io;

fn main() {
    println!("Which time from now(in secs) you want to shutdown your pc?");
    let mut text_input = String::new();
    io::stdin().read_line(&mut text_input).expect("Failed to read string from text input");
    let mut trimmed_text = text_input.trim();
    let time_input: u64 = trimmed_text.parse().expect("This is not an int");
    match trimmed_text.parse::<u64>() {
        Ok(i) => println!("Computer will shutdown after {} seconds", i),
        Err(..) => println!("This is not a valid time: {}", trimmed_text)
    }


    let hour = time::Duration::from_secs(time_input);
    let mut text_input = String::new();
    io::stdin().read_line(&mut text_input).expect("Failed to read string from text input");
    println!("{}", text_input);
    trimmed_text = text_input.trim();
    if (trimmed_text=="s"){
        thread::sleep(hour);
        match sleep() {
            Ok(_)=>println!("Shutting down!"),
            Err(error) => eprintln!("Failed to shutdown: {}", error)
        }
    }
    if (trimmed_text=="d"){
        thread::sleep(hour);
        match shutdown() {
            Ok(_)=>println!("Shutting down!"),
            Err(error) => eprintln!("Failed to shutdown: {}", error)
        }
    }
    else {
        println!("Thats not a valid func")
    }
}