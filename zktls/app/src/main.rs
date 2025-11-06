#![no_main]

mod binance;
mod errors;
mod phala;

pico_sdk::entrypoint!(main);
use crate::{
    binance::{ApiResponse, AssetData, BinanceData, PurchaseRecord, RedeemRecord},
    errors::{ZkErrorCode, ZktlsError},
    phala::{UserInfo, VmStatusMap},
};
use anyhow::{Context, Result, anyhow};
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
            "https://cloud-api.phala.network/api/v1/auth/me?",
            "https://www.binance.com/bapi/earn/v2/private/lending/union/purchaseRecord/list",
            "https://www.binance.com/bapi/earn/v1/private/lending/union/redemption/list?",
            "https://www.binance.com/bapi/earn/v1/private/finance-earn/position/group-by-asset?"
        ]
    });
    // 1. Verify
    let (attestation_data, _, messages) =
        verify_attestation_data(&attestation_data, &attestion_confg.to_string())?;
    println!("verify success");
    let source = extra_data_source(&attestation_data);
    println!("source is {source}");
    if (source.eq("phala")) {
        return handle_phala(&attestation_data);
    } else if (source.eq("binance")) {
        return handle_binance(&attestation_data);
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
        } else if url.contains("www.binance.com") {
            "binance"
        } else {
            "unknown"
        }
    } else {
        "unknown"
    }
}

fn handle_binance(attestation_data: &AttestationData) -> Result<()> {
    let first_public = attestation_data
        .public_data
        .get(0)
        .context("public_data is empty")?;
    let now_ts = first_public.attestationTime;
    let (today_start, yesterday_start, yesterday_end, day_before_start, day_before_end) =
        get_day_ranges(now_ts);

    //
    let default_token = "USDC";
    if let Some(responses) = attestation_data.private_data.plain_json_response.as_ref() {
        for response in responses {
            let mut today_asset: u64 = 0;
            if (response.id.eq("subscriptionList")) {
                let subscription_rsp: ApiResponse<Vec<PurchaseRecord>> =
                    serde_json::from_str(response.content.as_str())?;
                for sub in subscription_rsp.data {
                    println!("{} : {}", sub.asset, sub.amount);
                }
            }
            if (response.id.eq("redemptionList")) {
                let redeem_rsp: ApiResponse<Vec<RedeemRecord>> =
                    serde_json::from_str(response.content.as_str())?;
                for red in redeem_rsp.data {
                    println!("{} : {}", red.asset, red.amount);
                }
            }
            if (response.id.eq("assetDetails")) {
                let asset_data_rsp: ApiResponse<AssetData> =
                    serde_json::from_str(response.content.as_str())?;
                for asd in asset_data_rsp.data.asset_details {
                    if (default_token.eq(&asd.asset)) {
                        // today_asset = asd.amount.parse::<u64>()?;
                        break;
                    }
                }
            }
        }
    }
    Ok(())
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

fn get_day_ranges(now_ts: u64) -> (u64, u64, u64, u64, u64) {
    const SECS_PER_DAY: u64 = 24 * 60 * 60 * 1000;

    // today start
    let today_start = now_ts - (now_ts % SECS_PER_DAY);

    // yesterday start and end
    let yesterday_start = today_start - SECS_PER_DAY;
    let yesterday_end = today_start - 1;

    // the day before yesterday start and end
    let day_before_start = today_start - 2 * SECS_PER_DAY;
    let day_before_end = yesterday_start - 1;

    (
        today_start,
        yesterday_start,
        yesterday_end,
        day_before_start,
        day_before_end,
    )
}
pub fn main() {
    if let Err(e) = app_main() {
        println!("Error: {:?}", e);
        // panic or not?
        panic!("error {:?}", e);
    }
}
