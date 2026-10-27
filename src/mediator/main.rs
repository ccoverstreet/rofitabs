use std::io;
use serde_json::{Value, json};
use std::thread;
use std::net;
use std::sync::{Arc, Mutex};

use rofitabs::utils::{read_browser_msg, send_browser_msg, read_client_msg, send_client_msg};


fn main() -> io::Result<()> {
    let mut stdin = io::stdin();
    let stdout = Arc::new(Mutex::new(io::stdout()));

    let cur_tabs = Arc::new(Mutex::new(Value::Null));

    let cur_tabs_clone = Arc::clone(&cur_tabs);
    let stdout_clone = Arc::clone(&stdout);

    // Simple tcp listener for handling request from the client
    let _client_thread_handle = thread::spawn(move || {
        let listener = net::TcpListener::bind("127.0.0.1:23023").expect("Cannot bind to TCP port");

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

            let msg = read_client_msg(&mut stream).expect("Should be valid JSON string");
            match msg["id"].as_str().unwrap() {
                "list" => {
                    let cur_tabs = cur_tabs_clone.lock().unwrap();
                    let _ = send_client_msg(&mut stream, "tabs", cur_tabs.clone());
                },
                "activate" => {
                    let mut stdout = stdout_clone.lock().unwrap();
                    let _ = send_browser_msg(&mut stdout, "activate", msg["data"].clone());
                },
                &_ => {
                    let _ = send_client_msg(&mut stream, "empty", json!({}));
                }
            }
        }
    });

    loop {
        let msg = read_browser_msg(&mut stdin).expect("Should be valid JSON string");

        match msg["id"].as_str().unwrap() {
            "echo" => {
                let mut stdout = stdout.lock().unwrap();
                send_browser_msg(&mut stdout, "echo", msg)?;
            },
            "tabs" => {
                let mut cur_tabs = cur_tabs.lock().unwrap();
                *cur_tabs = msg["data"].clone();
            },
            &_ => todo!() // Just ignore, invalid message sent
        };

        
    }

    //let res = _client_thread_handle.join();

}




