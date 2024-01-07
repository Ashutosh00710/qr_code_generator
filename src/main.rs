mod bitset;
mod constants;
mod encoder;

use encoder::{
    DataEncoder,
    DataEncoderType::{Version10TO26, Version1TO9, Version27TO40},
};

fn main() {
    for data_encoder_type in [Version1TO9, Version10TO26, Version27TO40] {
        println!("version range: {:?}", data_encoder_type);
        let mut encoder = DataEncoder::new(data_encoder_type);
        let result = encoder.encode(String::from("HELLO WORLD").into_bytes());
        match result {
            Ok(r) => println!("result: {:?}", r.string()),
            Err(e) => println!("result: {}", e),
        }
        println!("-----------------------------")
    }
}
