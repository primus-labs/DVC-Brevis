use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct VmStatus {
    pub vm_uuid: String,
    pub status: String,
    pub uptime: String,
    pub in_progress: bool,
    pub boot_progress: Option<String>,
    pub boot_error: Option<String>,
    pub operation_type: Option<String>,
    pub operation_started_at: Option<String>,
    pub correlation_id: Option<String>,
}

pub type VmStatusMap = HashMap<String, VmStatus>;

