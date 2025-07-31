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
    // Extract the corresponding data according to the JSON path.
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
    } else if request_url
        == "https://www.binance.com/bapi/capital/v1/private/streamer/trade/get-user-trades"
    {
        {
            // Get the user id by `userId` or `userIdStr`
            let mut json_paths = vec![];
            json_paths.push("$.data[0].userId");
            json_paths.push("$.data[0].userIdStr");
            let json_value = messages[0].get_json_values(&json_paths);
            println!("userId:{:?}", json_value);
        }
        {
            // Obtain all `usdtAmount` values, accumulate them,
            // and then compare the sum with a base value.
            let mut json_paths = vec![];
            json_paths.push("$.data[*].usdtAmount");
            let json_value = messages[0].get_json_values(&json_paths);
            println!("usdtAmounts:{:?}", json_value);

            let usdt_total: f64 = json_value
                .unwrap()
                .iter()
                .map(|s| s.parse::<f64>().unwrap_or(0.0))
                .sum();
            println!("The total amount of USDT:{:?}", usdt_total);

            let base_value = 100.0;
            let result = usdt_total - base_value > 0.0;
            println!("Compared to the base value of {}:{:?}", base_value, result);
            commit(&result);
        }
    }

    commit(&attestation_data.public_data);
}
