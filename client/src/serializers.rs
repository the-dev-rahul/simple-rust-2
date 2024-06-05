use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct SignedMessage {
    data: Vec<f64>,
    signature: Vec<u8>,
    public_key: Vec<u8>,
}


impl SignedMessage{
    pub fn new( data: Vec<f64>, signature: Vec<u8>, public_key: Vec<u8> )-> SignedMessage{
        SignedMessage{
            data: data,
            signature: signature,
            public_key: public_key
        }
    }
}