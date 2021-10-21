use huelib::bridge;
use std::{thread, time, fs, net::{IpAddr}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Looking for bridges");
    let ip = bridge::discover_nupnp()?.pop().expect("found no bridges");
    clear();
    println!("Found bridge. Press Button to sync");

    let api_token = get_user_token(ip);
    clear();
    println!("Registered user: {}", api_token);
    Ok(())
}

/**
 */
fn get_user_token(ip: IpAddr) -> String {
    return fs::read_to_string("./user_token").unwrap_or_else(|_| { add_user(ip) })
}

fn add_user(ip: IpAddr) -> String {
    let token = bridge::register_user(ip, "polybar-hue").unwrap_or_else(|error| match error {
        huelib::Error::Response(response)
            if response.kind == huelib::response::ErrorKind::LinkButtonNotPressed =>
        {
            clear();
            println!("{}, Retrying in 5s ...", response);
            thread::sleep(time::Duration::from_secs(5));
            return add_user(ip);
        }
        _ => {
            panic!("crash and burn")
        }
    });

    fs::write("./user_token", &token).expect("Could not write user token to file");
    return token;
}

fn clear() {
    print!("{esc}c", esc = 27 as char);
}
