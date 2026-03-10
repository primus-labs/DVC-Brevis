#![no_main]
pico_sdk::entrypoint!(main);
use anyhow::{anyhow, Result};
use pico_sdk::io::{commit, read_as};
use serde_json::Value;
use zktls_att_verification::attestation_data::verify_attestation_data;

const ATTESTATION_CONFIG: &str = r#"{
  "attestor_addr": "0xe02bd7a6c8aa401189aebb5bad755c2610940a73",
  "url": [
    "https://www.binance.com/bapi/kyc/v2/private/certificate/user-kyc/current-kyc-status"
  ]
}"#;

fn app_main() -> Result<()> {
    let attestation_data: String = read_as();

    // 1. Verify
    let (attestation_data, _, messages) =
        verify_attestation_data(&attestation_data, ATTESTATION_CONFIG)?;
    commit(&attestation_data.public_data);

    // 2. Do some valid checks
    // Please handle it according to your actual business requirements.

    // Here is just a demonstration for checking request url.
    let request = attestation_data.public_data.request.clone();
    if request.url
        != "https://www.binance.com/bapi/kyc/v2/private/certificate/user-kyc/current-kyc-status"
    {
        return Err(anyhow!("Invalid request url!"));
    }

    // 3. Do some calculations and so on
    {
        // Get the kyc Status id by `data.kycStatus`
        let mut json_paths = vec![];
        json_paths.push("$.data.kycStatus");
        let kyc_status = messages[0].get_json_values(&json_paths)?;
        println!("kycStatus:{:?}", kyc_status);
        commit(&kyc_status);
    }

    Ok(())
}

pub fn main() {
    if let Err(e) = app_main() {
        println!("Error: {:?}", e);
        // panic or not?
        panic!("error {:?}", e);
    }
}
