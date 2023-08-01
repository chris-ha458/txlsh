use crate::consts::V_TABLE;

/// Exposed to Python as pearson_hash
pub(crate) fn pearson_h(salt: u8, ii: u8, jj: u8, kk: u8) -> u8 {
    let mut h = V_TABLE[salt as usize];
    h = V_TABLE[(h ^ ii) as usize];
    h = V_TABLE[(h ^ jj) as usize];
    V_TABLE[(h ^ kk) as usize]
}
