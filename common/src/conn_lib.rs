use std::{io::{Write, Read}, net::TcpStream, sync::{Arc, Mutex}};

use openssl::ssl::SslStream;



pub fn write_flush_client(stream: Arc<Mutex<SslStream<TcpStream>>>, msg: String) -> Result<(), std::io::Error> {
    // Pad Message to 1024 bytes
    let mut msg_data = [0u8;1024];
    for (i,b) in msg.bytes().into_iter().enumerate() {
        msg_data[i] = b;
    }
    // Write Message
    let mut writer = stream.lock().expect("Failed to lock stream");
    let s = writer.write_all(&msg_data);
    writer.flush()?;
    s
}

pub fn read_stream_client(stream: Arc<Mutex<SslStream<TcpStream>>>) -> Result<String, std::io::Error> {
    let mut reader = stream.lock().expect("Failed to lock stream");
    let mut buf = [0u8; 1024];
    reader.read_exact(&mut buf)?;
    let msg = String::from_utf8(buf.to_vec()).unwrap();
    Ok(String::from(msg.trim_matches(char::from(0))))
}


pub async fn write_flush(stream: Arc<Mutex<SslStream<TcpStream>>>, msg: String) -> Result<(), std::io::Error> {
    // Pad Message to 1024 bytes
    let mut msg_data = [0u8;1024];
    for (i,b) in msg.bytes().into_iter().enumerate() {
        msg_data[i] = b;
    }
    // Write Message
    let mut writer = stream.lock().expect("Failed to lock stream");
    let s = writer.write_all(&msg_data);
    writer.flush()?;
    s
}

pub async fn read_stream(stream: Arc<Mutex<SslStream<TcpStream>>>) -> Result<String, std::io::Error> {
    let mut reader = stream.lock().expect("Failed to lock stream");
    let mut buf = [0u8; 1024];
    reader.read_exact(&mut buf)?;
    let msg = String::from_utf8(buf.to_vec()).unwrap();
    Ok(String::from(msg.trim_matches(char::from(0))))
}