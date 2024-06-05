use std::net::TcpStream;
use std::io::Write;
use tokio_tungstenite::connect_async;
use futures_util::StreamExt;
use url::Url;
use std::time::SystemTime;
use tokio::runtime::Runtime;


mod singnature;
mod serializers;

async fn fetch_and_send_to_aggregator(stream: &mut TcpStream) {

    let times: u64 = 10;
    let url = Url::parse("wss://stream.binance.com:9443/ws/btcusdt@trade").unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    let (_write, mut read) = ws_stream.split();

    let start_time = SystemTime::now();
    let mut prices : Vec<f64> = Vec::new();

    while let Some(message) = read.next().await  {     
        
        let message = message.unwrap().into_text().unwrap();
        let value: Result<serde_json::Value, serde_json::Error> = serde_json::from_str(&message);
        match value {
            Ok(data) => {
                let s = data["p"].to_string().replace("\"","");
                match s.parse::<f64>() {
                    Ok(num) => prices.push(num),
                    Err(e) => println!("Failed to parse string to f64: {}", e),
                }
            },
            Err(e) => {
                eprintln!("Failed to parse JSON: {}", e);
            }
        }
        
        let end_time = SystemTime::now();
        let duration_utc = end_time.duration_since(start_time).expect("Time went backwards");
        let duration: u64 = duration_utc.as_secs(); 

        if duration>=times  {
            break;
        }
    }

    let singnature_manager = singnature::SignatureManager::new();

    let signed_message = serializers::SignedMessage::new(
        prices.clone(),
        singnature_manager.sign(&prices),
        singnature_manager.public_key(),
    );

    let serialized_message = serde_json::to_string(&signed_message).unwrap();

    let stream_data = stream.write_all(serialized_message.as_bytes());

    println!("stream {:?}", stream_data);
}


fn main() {


    let rt = Runtime::new().unwrap();
    rt.block_on(async {

        match TcpStream::connect("127.0.0.1:7878") {
            Ok(mut stream) => {
                fetch_and_send_to_aggregator(&mut stream).await;
            }
            Err(e) => {
                eprintln!("Failed to connect to aggregator: {}", e);
            }
        }
    });
}
