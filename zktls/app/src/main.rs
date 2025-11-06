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
use zktls_att_verification::attestation_data::{AttestationData, verify_attestation_data};

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
    let source = extra_data_source(&attestation_data);
    println!("source is ${source}");
    if (source.eq("phala")) {
        return handle_phala(&attestation_data);
    } else if (source.eq("binance")) {
    } else {
        ensure_zk!(true, zkerr!(ZkErrorCode::NotSupportSource));
    }
    Ok(())
}

// Extra source from attestation
fn extra_data_source(attestation_data: &AttestationData) -> &'static str {
    if let Some(request) = attestation_data
        .public_data
        .get(0)
        .and_then(|pd| pd.attestation.request.get(0))
    {
        let url = &request.url;

        if url.contains("cloud-api.phala.network") {
            "phala"
        } else if url.contains("api.binance.com") {
            "binance"
        } else {
            "unknown"
        }
    } else {
        "unknown"
    }
}

fn handle_phala(attestation_data: &AttestationData) -> Result<()> {
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
                up_time_enough = check_all_vm_uptime(&vms);
            }
        }

        ensure_zk!(up_time_enough, zkerr!(ZkErrorCode::UpTimeNotEnough));
    } else {
        ensure_zk!(true, zkerr!(ZkErrorCode::EmptyPlainResponse));
    }
    Ok(())
}

fn check_all_vm_uptime(vms: &VmStatusMap) -> bool {
    let mut total_seconds: u64 = 0;

    for (_uuid, vm_status) in vms {
        let uptime = &vm_status.uptime;
        // if time unit is hour and others, uptime meets the requirement
        if uptime.contains("day")
            || uptime.contains("days")
            || uptime.contains("hour")
            || uptime.contains("h")
            || uptime.contains("hours")
            || uptime.contains("month")
            || uptime.contains("months")
            || uptime.contains("year")
            || uptime.contains("years")
        {
            return true;
        } 
        // Compute total time of cvms
        total_seconds += parse_minutes_seconds(uptime);
    }
    println!("VM uptime: {}", total_seconds);
    total_seconds >= 10 * 60
}

fn parse_minutes_seconds(uptime: &str) -> u64 {
    let mut seconds = 0u64;

    for part in uptime.split_whitespace() {
        if part.ends_with('m') {
            if let Ok(n) = part.trim_end_matches('m').parse::<u64>() {
                seconds += n * 60;
            }
        } else if part.ends_with('s') {
            if let Ok(n) = part.trim_end_matches('s').parse::<u64>() {
                seconds += n;
            }
        }
    }

    seconds
}

pub fn main() {
    if let Err(e) = app_main() {
        println!("Error: {:?}", e);
        // panic or not?
        panic!("error {:?}", e);
    }
}
