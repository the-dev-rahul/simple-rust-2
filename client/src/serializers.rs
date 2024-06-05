use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SignedMessage {
    message: Vec<f64>,
    signature: Vec<u8>,
    public_key: Vec<u8>,
}


impl SignedMessage{
    pub fn new( message: Vec<f64>, signature: Vec<u8>, public_key: Vec<u8> )-> SignedMessage{
        SignedMessage{
            message: message,
            signature: signature,
            public_key: public_key
        }
    }
}