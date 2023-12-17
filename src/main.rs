mod constants;
mod bitset;
mod encoder;

use encoder::{
    DataEncoderType::{
        Version1TO9,
        Version10TO26,
        Version27TO40,
    },
    DataEncoder
};

fn main() {
    for data_encoder_type in [Version1TO9, Version10TO26, Version27TO40] {
        let mut encoder = DataEncoder::new(data_encoder_type);
        let result = encoder.encode(String::from("HTTPS://GOOGLE.COM").into_bytes());
        match result {
            Ok(r)=> println!("{:?}", r.value()),
            Err(e) => println!("{:?}", e),
        }
    }
}
