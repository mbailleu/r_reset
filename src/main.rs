use rust_gpiozero::*;
use clap::{Arg, App}; 
use std::thread;
use std::time::Duration;

fn reset(port: u8, time: u64) {
    let led = LED::new(port);
    led.on();
    thread::sleep(Duration::from_secs(time));
    led.off();
}

fn main() {
    let matches = App::new("Reset program for servers in Edinburgh")
        .version("0.1.0")
        .arg(Arg::with_name("server")
            .required(true)
            .help("Server to send reset"))
        .arg(Arg::with_name("time")
             .help("Time to send requests defaults to 5"))
        .get_matches();

    let server = matches.value_of("server").unwrap_or("-1");
    let time : u64 = matches.value_of("time").unwrap_or("5").parse().unwrap();
    println!("The server you chose is: {}", server);

    let port : i64 = match server {
        "rose" => 17,
        "martha" => 27,
        "donna" => 22,
        "clara" => 23,
        "amy" => 24,
        _ => -1
    };

    if port != -1 && port < 256 {
        println!("Port is {}", port);
        println!("Reset...");
        reset(port as u8, time);
        println!("Done.");
        return;
    }
    println!("Server {} unknown", server);
    std::process::exit(1);
}
