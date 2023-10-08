use system_shutdown::shutdown;
use std::{thread, time};

fn main() {
    let hour = time::Duration::from_secs(3600);
    thread::sleep(hour);
    match shutdown() {
        Ok(_)=>println!("Shutting down!"),
        Err(error) => eprintln!("Failed to shutdown: {}", error)
    }
}
