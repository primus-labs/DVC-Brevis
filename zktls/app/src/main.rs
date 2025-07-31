#![no_main]
pico_sdk::entrypoint!(main);
use anyhow::{Result, anyhow};
use pico_sdk::io::{commit, read_as};
use serde_json::Value;
use zktls_att_verification::attestation_data::verify_attestation_data;

fn app_main() -> Result<()> {
    let attestation_data: String = read_as();
    let attestation_config: String = read_as();

    // 1. Verify
    let (attestation_data, _, messages) =
        verify_attestation_data(&attestation_data, &attestation_config)?;
    commit(&attestation_data.public_data);

    // 2. Do some valid checks
    // Please handle it according to your actual business requirements.
    // Here is just a demonstration.
    let request = attestation_data.public_data.request.clone();
    let request_body: Value = serde_json::from_str(&request.body)?;
    let base_asset = request_body["baseAsset"].as_str().unwrap();
    let start_time = request_body["startTime"].as_i64().unwrap();
    let end_time = request_body["endTime"].as_i64().unwrap();

    if base_asset != "BNB" {
        return Err(anyhow!("Invalid base asset!"));
    }

    const MIN_END_TIME: i64 = 1752969600000; // 2025-07-20 00:00:00 UTC+0
    commit(&MIN_END_TIME);
    if end_time < MIN_END_TIME {
        return Err(anyhow!("Not within the specified date range!"));
    }

    const MAX_DURATION_MS: i64 = 32 * 24 * 60 * 60 * 1000; // 32 days
    commit(&MAX_DURATION_MS);
    if end_time - start_time >= MAX_DURATION_MS {
        return Err(anyhow!("The date range is too large!"));
    }

    if request.url
        != "https://www.binance.com/bapi/capital/v1/private/streamer/trade/get-user-trades"
    {
        return Err(anyhow!("Invalid request url!"));
    }

    // 3. Do some calculations and so on
    {
        // Get the user id by `userId`
        let mut json_paths = vec![];
        json_paths.push("$.data[0].userId");
        let user_id = messages[0].get_json_values(&json_paths)?;
        println!("userId:{:?}", user_id);
        commit(&user_id);
    }

    {
        // Obtain all `usdtAmount` values, accumulate them,
        // and then compare the sum with a base value.
        let mut json_paths = vec![];
        json_paths.push("$.data[*].usdtAmount");
        let usdt_amounts = messages[0].get_json_values(&json_paths)?;
        println!("usdtAmounts:{:?}", usdt_amounts);

        let usdt_total: f64 = usdt_amounts
            .iter()
            .map(|s| s.parse::<f64>().unwrap_or(0.0))
            .sum();
        println!("The total amount of USDT:{:?}", usdt_total);

        const BASE_VALUE: f64 = 1000.0;
        let res = (usdt_total - BASE_VALUE) > 0.0;
        println!("Compared to the base value of {}:{:?}", BASE_VALUE, res);
        commit(&BASE_VALUE);
        commit(&res);
        if !res {
            return Err(anyhow!("Not reach the minimum transaction amount!"));
        }
    }

    Ok(())
}

pub fn main() {
    if let Err(e) = app_main() {
        println!("Error: {:?}", e);
        // panic or not?
    }
}
