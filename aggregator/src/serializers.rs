use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SignedMessage {
    pub message: Vec<f64>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>,
}


impl SignedMessage{
    pub fn _new( message: Vec<f64>, signature: Vec<u8>, public_key: Vec<u8> )-> SignedMessage{
        SignedMessage{
            message: message,
            signature: signature,
            public_key: public_key
        }
    }
}