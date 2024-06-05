use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufRead};
use serde_json;

mod singnature;
mod serializers;


fn handle_client(prices: &mut Vec<f64>,stream: TcpStream) {

    let reader: BufReader<TcpStream> = BufReader::new(stream);

    for line in reader.lines() {
        let line = line.unwrap();

        let mut signed_message: serializers::SignedMessage = serde_json::from_str(&line).unwrap();
        let serialized_data = serde_json::to_string(&signed_message.data).unwrap();

        let is_verified: bool= singnature::SignatureManager::verify(&signed_message.public_key, &serialized_data.as_bytes(), &signed_message.signature);

        if is_verified{
            prices.append(&mut signed_message.data);
            let average_price: f64 = prices.iter().sum::<f64>() / prices.len() as f64;
            println!("new average price: {}", average_price);
        } else {
            eprintln!("Signature verification failed");
        }
    }

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut prices: Vec<f64> = Vec::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_client(&mut prices, stream);
    }
}
