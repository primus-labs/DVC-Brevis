#![no_main]

mod errors;
mod phala;

pico_sdk::entrypoint!(main);
use crate::{
    errors::{ZkErrorCode, ZktlsError},
    phala::{UserInfo, VmStatusMap},
};
use anyhow::{Result, anyhow};
use pico_sdk::io::{commit, read_as};
use regex::Regex;
use serde_json::{Value, json};
use zktls_att_verification::attestation_data::verify_attestation_data;

fn app_main() -> Result<()> {
    let attestation_data: String = read_as();
    println!("attestation_data:{}", attestation_data);

    // 0. Make attestation config
    let v: serde_json::Value = serde_json::from_str(&attestation_data)
        .map_err(|e| zkerr!(ZkErrorCode::ParseAttestationData, e.to_string()))?;
    let attestor_addr = v
        .get("public_data")
        .and_then(|pd| pd.get(0))
        .and_then(|item| item.get("attestor"))
        .and_then(|a| a.as_str())
        .ok_or_else(|| zkerr!(ZkErrorCode::GetAttestorAddressFail))?;
    let attestion_confg = json!({
        "attestor_addr": attestor_addr,
        "url": ["https://cloud.phala.network/api/status/batch",
            "https://cloud-api.phala.network/api/v1/auth/me?"
        ]
    });
    // 1. Verify
    let (attestation_data, _, messages) =
        verify_attestation_data(&attestation_data, &attestion_confg.to_string())?;
    println!("verify success");

    commit(&attestation_data.public_data);

    // Please handle it according to your actual business requirements.
    // Here is just a demonstration.
    if let Some(responses) = attestation_data.private_data.plain_json_response.as_ref() {
        let mut up_time_enough = false;

        for response in responses {
            let id = response.id.as_str();
            let content = response.content.as_str();
            if ("userInfo".eq(id)) {
                let user_info: UserInfo = serde_json::from_str(content)?;
                println!("userInfo: {:?}", user_info);
                commit(&user_info.email);
            } else {
                let vms: VmStatusMap = serde_json::from_str(content)?;
                for (uuid, vm_status) in &vms {
                    println!("VM UUID: {}", uuid);
                    println!("Uptime: {}", vm_status.uptime);
                    println!("---------------------------");
                    if check_uptime(&vm_status.uptime) {
                        up_time_enough = true;
                        break;
                    }
                }

                if up_time_enough {
                    break;
                }
            }
        }

        ensure_zk!(up_time_enough, zkerr!(ZkErrorCode::UpTimeNotEnough));
    } else {
        ensure_zk!(true, zkerr!(ZkErrorCode::EmptyPlainResponse));
    }

    Ok(())
}

fn check_uptime(uptime: &str) -> bool {
    if uptime.contains("years")
        || uptime.contains("year")
        || uptime.contains("months")
        || uptime.contains("month")
        || uptime.contains("days")
        || uptime.contains("day")
        || uptime.contains("hours")
        || uptime.contains("hour")
    {
        return true;
    }

    if let Some(pos) = uptime.find("minutes") {
        let num_str: String = uptime[..pos]
            .chars()
            .rev()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .chars()
            .rev()
            .collect();

        if let Ok(minutes) = num_str.parse::<u64>() {
            return minutes > 10;
        }
    }
    false
}

pub fn main() {
    if let Err(e) = app_main() {
        println!("Error: {:?}", e);
        // panic or not?
        panic!("error {:?}", e);
    }
}
