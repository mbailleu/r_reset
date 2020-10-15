use rust_gpiozero::*;
use clap::{Arg, App}; 
use std::thread;
use std::time::Duration;

fn reset(port: u8) {
    let led = LED::new(port);
    led.on();
    thread::sleep(Duration::from_secs(5));
    led.off();
}

fn main() {
    let matches = App::new("Reset program for servers in Edinburgh")
        .version("0.1.0")
        .arg(Arg::with_name("port")
            .required(true)
            .help("Port to send reset"))
        .get_matches();

    let server = matches.value_of("port").unwrap_or("17");
    println!("The server you chose is: {}", server);

    let port : i64 = match server {
        "rose" => 0,
        "martha" => 1,
        "donna" => 2,
        "clara" => 3,
        "amy" => 4,
        _ => -1
    };

    if port != -1 && port < 256 {
        println!("Port is {}", port);
        println!("Reset...");
        reset(port as u8);
        println!("Done.");
        return;
    }
    println!("Server {} unknown", server);
    std::process::exit(1);
}
