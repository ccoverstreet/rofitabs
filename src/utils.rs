use std::io::{Read, Write};
use serde_json::{json};

pub fn read_browser_msg(stdin: &mut std::io::Stdin) -> serde_json::Result<serde_json::Value> {
    let mut header_buf: [u8; 4] = [0; 4];
    stdin.read_exact(&mut header_buf).expect("Header should be 4 bytes of a u32 int"); // Blocks until buffer is filled

    let n_body = u32::from_ne_bytes(header_buf).try_into().expect("Should be a u32 int");
    let mut body_buf = vec![0; n_body];
    stdin.read_exact(&mut body_buf).expect("Can't read body of browser message");

    let json_content_str = 
        str::from_utf8(&body_buf).expect("Must be a valid UTF-8 string");


    serde_json::from_str(json_content_str)
}

pub fn send_browser_msg(stdout: &mut std::io::Stdout, id: &str, val: serde_json::Value) -> Result<(), std::io::Error> {
    let msg = json!({
        "id": id,
        "data": val
    });

    let body_string = msg.to_string();
    let body = body_string.as_bytes();
    let len = u32::to_ne_bytes(body.len() as u32);
    stdout.write(&len)?;
    stdout.write(&body)?;
    stdout.flush()?;

    Ok(())
}

pub fn read_client_msg(stream: &mut std::net::TcpStream) -> serde_json::Result<serde_json::Value> {
    let mut header_buf: [u8; 4] = [0; 4];
    stream.read_exact(&mut header_buf).expect("Header should be 4 bytes of a u32 int"); // Blocks until buffer is filled

    let n_body = u32::from_ne_bytes(header_buf).try_into().expect("Should be a u32 int");
    let mut body_buf = vec![0; n_body];
    stream.read_exact(&mut body_buf).expect("Can't read body of client message");

    let json_content_str = 
        str::from_utf8(&body_buf).expect("Must be a valid UTF-8 string");

    serde_json::from_str(json_content_str)
}

pub fn send_client_msg(stream: &mut std::net::TcpStream, id: &str, val: serde_json::Value) -> Result<(), std::io::Error> {
    let msg = json!({
        "id": id,
        "data": val
    });

    let body_string = msg.to_string();
    let body = body_string.as_bytes();
    let len = u32::to_ne_bytes(body.len() as u32);

    stream.write(&len)?;
    stream.write(&body)?;
    stream.flush()?;

    Ok(())
}

