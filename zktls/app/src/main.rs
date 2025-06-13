#![no_main]
pico_sdk::entrypoint!(main);

use pico_sdk::io::{commit, read_as};
use zktls_att_verification::attestation_data::{AttestationConfig, AttestationData};

pub fn main() {
    let attestation_data: String = read_as();
    let attestation_config: String = read_as();
    // println!("attestation_data {:?}", attestation_data);
    // println!("attestation_config {:?}", attestation_config);

    let attestation_data: AttestationData = serde_json::from_str(&attestation_data).unwrap();
    let attestation_config: AttestationConfig = serde_json::from_str(&attestation_config).unwrap();

    // Verify
    let messages = attestation_data.verify(&attestation_config).unwrap();

    // Here is just a demonstration.
    // Please handle it according to your actual business requirements.
    let request_url = attestation_data.public_data.request.url.clone();
    let mut json_paths = vec![];
    if request_url == "https://www.bitget.com/v1/mix/vip/need" {
        json_paths.push("$.data.spotVol");
        let json_value = messages[0].get_json_values(&json_paths);
        println!("data.spotVol:{:?}", json_value);
    } else if request_url == "https://www.bitget.com/v1/spot/order/historyList" {
        json_paths.push("$.data.data");
        let json_value = messages[0].get_json_values(&json_paths);
        println!("data.data:{:?}", json_value);
    }

    commit(&attestation_data.public_data);
}
