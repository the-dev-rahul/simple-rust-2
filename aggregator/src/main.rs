use std::net::{TcpListener, TcpStream};
use std::io::{Read};
use std::sync::{Arc, Mutex};
use std::thread;

fn handle_client(mut stream: TcpStream, _averages: Arc<Mutex<Vec<f64>>>) {
    let mut buffer = [0; 100000];
    let mut prices: Vec<f64> = Vec::new();

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let json = String::from_utf8_lossy(&buffer[..bytes_read]).trim().to_string();
            let mut new_prices: Vec<f64> = serde_json::from_str(&json).unwrap();
            prices.append(&mut new_prices);
            let average_price: f64 = prices.iter().sum::<f64>() / prices.len() as f64;
            println!("new average price: {}", average_price);

        }
        Err(e) => {
            println!("Failed to read from stream: {}", e);
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let averages = Arc::new(Mutex::new(Vec::new()));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let averages = Arc::clone(&averages);

        thread::spawn(move || {
            handle_client(stream, averages);
        });
    }
}
