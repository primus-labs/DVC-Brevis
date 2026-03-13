#![no_main]
pico_sdk::entrypoint!(main);
use anyhow::Result;
use pico_sdk::io::{commit, read_as};
use zktls_att_verification::attestation_data::verify_attestation_data;

// Different data sources use different configuration parameters.
// Refer to `zktls/data/xxx_config.json` and replace the content of ATTESTATION_CONFIG.
const ATTESTATION_CONFIG: &str = r#"{
  "attestor_addr": "0xe02bd7a6c8aa401189aebb5bad755c2610940a73",
  "url": [
    "https://github.com/_global-navigation/payloads.json",
    "https://github.com"
  ],
  "conditions": [
    {
      "op": "REVEAL_SALTTED_HASH",
      "field": "{\"type\":\"FIELD_ARITHMETIC\",\"op\":\"SHA256_WITH_SALT\",\"field\":\"$.owner.avatarUrl\"}",
      "reveal_id": "github_id"
    },
    {
      "op": "REVEAL_SALTTED_HASH",
      "field": "{\"type\":\"FIELD_ARITHMETIC\",\"op\":\"SHA256_WITH_SALT\",\"field\":\"//h2[@id=\\\"js-contribution-activity-description\\\"]\"}",
      "reveal_id": "contribution"
    },
    {
      "op": "REVEAL_SALTTED_HASH",
      "field": "{\"type\":\"FIELD_ARITHMETIC\",\"op\":\"SHA256_WITH_SALT\",\"field\":\"//div[@id=\\\"year-list-container\\\"]//ul[@class=\\\"filter-list small\\\"]/li[*]/a\"}",
      "reveal_id": "years"
    },
    {
      "op": "REVEAL_SALTTED_HASH",
      "field": "{\"type\":\"FIELD_ARITHMETIC\",\"op\":\"SHA256_WITH_SALT\",\"field\":\"//a[@id=\\\"year-link-2026\\\"]/@data-hydro-click\"}",
      "reveal_id": "github_id_in_html"
    }
  ]
}"#;

fn app_main() -> Result<()> {
    let attestation_data: String = read_as();

    // 1. Verify
    let (attestation_data, _, _messages) =
        verify_attestation_data(&attestation_data, ATTESTATION_CONFIG)?;
    commit(&attestation_data.public_data);

    // 2. Do some valid checks
    // Please handle it according to your actual business requirements.

    {
        // using the private_data
        let private_data = attestation_data.private_data.clone();
        for _ in private_data {
            //
        }
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
