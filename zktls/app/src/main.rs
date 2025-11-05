#![no_main]

mod errors;
mod phala;

pico_sdk::entrypoint!(main);
use crate::{
    errors::{ZkErrorCode, ZktlsError},
    phala::VmStatusMap,
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
        "url": ["https://cloud.phala.network/api/status/batch?"
        ]
    });
    // 1. Verify
    let (attestation_data, _, messages) =
        verify_attestation_data(&attestation_data, &attestion_confg.to_string())?;
    println!("verify success");

    commit(&attestation_data.public_data);

    // Please handle it according to your actual business requirements.
    // Here is just a demonstration.
    if let Some(first_response) = attestation_data
        .private_data
        .plain_json_response
        .as_ref()
        .and_then(|v| v.get(0))
    {
        let content = first_response.content.clone();
        let vms: VmStatusMap = serde_json::from_str(&content)?;

        let mut up_time_enough = false;
        for (uuid, vm_status) in &vms {
            println!("VM UUID: {}", uuid);
            println!("Uptime: {}", vm_status.uptime);
            println!("---------------------------");
            if let Some(minutes) = parse_uptime(&vm_status.uptime) {
                if (minutes > 10) {
                    println!("up minutes:{}",minutes);
                    up_time_enough = true;
                    break;
                }
            }
        }
        ensure_zk!(up_time_enough, zkerr!(ZkErrorCode::UpTimeNotEnough));
    } else {
        ensure_zk!(true,zkerr!(ZkErrorCode::EmptyPlainResponse));
    }

    Ok(())
}

fn parse_uptime(uptime: &str) -> Option<u64> {
    // Extra years, months, days, hours, minutes, seconds
    let re = Regex::new(r"(?:(?P<years>\d+)y)?(?:(?P<months>\d+)mo?)?(?:(?P<days>\d+)d)?(?:(?P<hours>\d+)h)?(?:(?P<minutes>\d+)m)?(?:(?P<seconds>\d+)s)?").unwrap();
    if let Some(caps) = re.captures(uptime) {
        let years: u64 = caps
            .name("years")
            .map_or(0, |m| m.as_str().parse().unwrap_or(0));
        let months: u64 = caps
            .name("months")
            .map_or(0, |m| m.as_str().parse().unwrap_or(0));
        let days: u64 = caps
            .name("days")
            .map_or(0, |m| m.as_str().parse().unwrap_or(0));
        let hours: u64 = caps
            .name("hours")
            .map_or(0, |m| m.as_str().parse().unwrap_or(0));
        let minutes: u64 = caps
            .name("minutes")
            .map_or(0, |m| m.as_str().parse().unwrap_or(0));
        let seconds: u64 = caps
            .name("seconds")
            .map_or(0, |m| m.as_str().parse().unwrap_or(0));

        // Convert to minutes
        Some(
            years * 365 * 24 * 60
                + months * 30 * 24 * 60
                + days * 24 * 60
                + hours * 60
                + minutes
                + seconds / 60,
        )
    } else {
        None
    }
}

pub fn main() {
    if let Err(e) = app_main() {
        println!("Error: {:?}", e);
        // panic or not?
        panic!("error {:?}", e);
    }
}
