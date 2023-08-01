use std::{fmt::Display, num::ParseIntError};

/// An enum for possible errors that might occur while calculating hash values.
#[derive(Debug)]
pub enum TxLshError {
    /// Input's length is too big to handle. Maximal file size is 4GB.
    DataLenOverflow,
    /// The hash string is malformed and cannot be parsed.
    InvalidHashValue,
    /// TLSH requires an input of at least 50 bytes.
    MinSizeNotReached,
    /// Fails to parse a hex string to integer.
    ParseHexFailed,
    // No valid hash found. See https://github.com/trendmicro/tlsh/issues/79
    NoValidHash,
}

impl From<ParseIntError> for TxLshError {
    fn from(_: ParseIntError) -> Self {
        Self::ParseHexFailed
    }
}

impl Display for TxLshError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TxLshError::DataLenOverflow => {
                write!(f, "Input file is too big. Maximal file size is 4GB.")
            }
            TxLshError::InvalidHashValue => write!(f, "Can't parse hash string"),
            TxLshError::MinSizeNotReached => {
                write!(f, "TLSH requires an input of at least 50 bytes.")
            }
            TxLshError::ParseHexFailed => write!(f, "Can't convert hex string to integer"),
            TxLshError::NoValidHash => write!(
                f,
                "No valid hash could be computed. See https://github.com/trendmicro/tlsh/issues/79"
            ),
        }
    }
}
