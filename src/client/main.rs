use std::io;
use std::fs;
use std::path::{PathBuf};
use serde_json::{Value, json};
use std::net::{TcpStream};
use std::env;

use rofitabs::utils::{read_client_msg, send_client_msg};

const NATIVE_MANIFEST_JSON: &str= include_str!("../../extension/native.json");

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args[1] == "install" {
        // Install native app manifests

        let home = env::var("HOME").expect("HOME not set");
        let final_str = NATIVE_MANIFEST_JSON.to_string().replace("$HOME", &home);
        let path = PathBuf::from(home).join(".mozilla/native-messaging-hosts/rofitabs.json");
        println!("Wrote native app manifest to {}", path.display());
        fs::write(path, final_str)?;

        return Ok(());
    }

    let mut stream = TcpStream::connect("localhost:23023").expect("Connection to mediator failed");

    if args.len() < 2 {
        println!("list");
        send_client_msg(&mut stream, "list", json!({}))?;
        let res = read_client_msg(&mut stream).expect("Unable to receive message for list command");
        rofi_print(&res["data"]);
        return Ok(());
    }

    if args[1] == "list" {
        println!("list");
        send_client_msg(&mut stream, "list", json!({}))?;
        let res = read_client_msg(&mut stream).expect("Unable to receive message for list command");
        rofi_print(&res["data"]);
    } else if args[1] == "activate" {
        if args.len() < 3 {
            println!("Must provide a tab id to activate");
            return Ok(());
        }

        send_client_msg(&mut stream, "activate", json!(args[2]))?
    } else { // Assume that ID is the only provided argument
        send_client_msg(&mut stream, "activate", json!(args[1]))?
    }

    Ok(())
}

fn rofi_print(tabs: &serde_json::Value) {
    //let tabs_arr: Vec<Value>= tabs["data"].as_array().expect("Should be array").as_array().unwrap().to_vec();
    let tabs_arr: Vec<Value>= tabs.as_array().expect("Should be array").to_vec();
    for t in tabs_arr.iter() {
        println!("{} {}", t["id"], t["title"]);
    }
}

