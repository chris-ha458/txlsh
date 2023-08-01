use crate::consts::V_TABLE;
use crate::helper::Version;
use xxhash_rust;

/// takes four u8 values and version and apply pearson or xxhash_h

pub(crate) fn hasher(salt: u8, ii: u8, jj: u8, kk: u8, ver: Version) -> u8 {
    match ver {
        Version::TxLshV1 => xxhash_h(salt, ii, jj, kk),
        _ => pearson_h(salt, ii, jj, kk),
    }
}

/// Exposed to Python as pearson_hash
pub(crate) fn pearson_h(salt: u8, ii: u8, jj: u8, kk: u8) -> u8 {
    let mut h = V_TABLE[salt as usize];
    h = V_TABLE[(h ^ ii) as usize];
    h = V_TABLE[(h ^ jj) as usize];
    V_TABLE[(h ^ kk) as usize]
}

/// same interface as pearson_h
/// invokes xxh3_64 and truncates into u8
pub(crate) fn xxhash_h(salt: u8, ii: u8, jj: u8, kk: u8) -> u8 {
    xxhash_rust::xxh3::xxh3_64(&[salt, ii, jj, kk]) as u8
}
