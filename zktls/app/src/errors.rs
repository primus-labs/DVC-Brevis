use std::fmt;
use std::fmt::{Debug, Formatter};

#[repr(i16)]
#[derive(Clone, Debug)]
pub enum ZkErrorCode {
    ParseAttestationData = 1001,
    GetAttestorAddressFail,
    VerifyAttestation,
    InvalidRequestLength,
    InvalidMessagesLength,
    GetJsonValueFail,
    InvalidJsonValueSize,
    CannotFoundTimestamp,
    ParseTimestampFailed,
    InvalidRequestOrder,
    InvalidRequestUrl,
    DuplicateAccount,
    EmptyPlainResponse,
    UpTimeNotEnough
}

pub struct ZktlsError {
    code: ZkErrorCode,
    msg: String,
}

impl ZktlsError {
    pub fn new(code: ZkErrorCode, msg: impl Into<String>) -> Self {
        Self {
            code,
            msg: msg.into(),
        }
    }
    pub fn icode(&self) -> i16 {
        self.code.clone() as i16
    }
    pub fn msg(&self) -> String {
        self.msg.clone()
    }
}

// Implementing the std::fmt::Display trait for custom error message formatting
impl fmt::Display for ZktlsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error {}: {}", self.icode(), self.msg())
    }
}

impl Debug for ZktlsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

// Implementing the std::error::Error trait
impl std::error::Error for ZktlsError {}

#[macro_export]
macro_rules! ensure_zk {
    ($cond:expr, $err:expr) => {
        if !$cond {
            return Err($err.into());
        }
    };
}

#[macro_export]
macro_rules! zkerr {
    ($code:expr, $msg:expr) => {
        ZktlsError::new($code, $msg)
    };
    ($code:expr) => {
        zkerr!($code, stringify!($code).to_string())
    };
}
