mod bitset;
mod constants;
mod encoder;
mod version;

use crate::version::ErrorCorrectionLevels;
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
            Ok(r) => {
                println!("result: {:?}", r.string());
                if let Some(qr_code_version) = version::choose_qr_code_version(
                    ErrorCorrectionLevels::Medium,
                    &encoder,
                    r.len(),
                ) {
                    println!("selected version: {}", qr_code_version.version);
                    break;
                }
            }
            Err(e) => println!("result: {}", e),
        }
        println!("-----------------------------")
    }
}
